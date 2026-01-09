use anyhow::{Context, Result};

use burn_lm_router::BurnLmRouter;
use prompt_management_agent::{AgentConfig, PromptManagementAgent, RefinedPrompt, ZedContext};
use pyo3_bridge::{ProxyRequest, PyO3Bridge};

use crate::config::WidgetLogConfig;
use crate::lifecycle::WidgetLogProcess;

/// Integrated prompt manager that orchestrates all layers
pub struct IntegratedPromptManager {
    management_agent: Option<PromptManagementAgent>,
    pyo3_bridge: Option<PyO3Bridge>,
    burn_router: BurnLmRouter,
    widget_log_process: Option<WidgetLogProcess>,
    config: WidgetLogConfig,
}

impl IntegratedPromptManager {
    /// Create a new integrated prompt manager
    pub async fn new(config: WidgetLogConfig) -> Result<Self> {
        log::info!("Initializing IntegratedPromptManager");

        // Initialize Burn-LM router (heuristic mode, no backend needed in Phase 2a)
        let burn_router = BurnLmRouter::new();

        Ok(Self {
            management_agent: None,
            pyo3_bridge: None,
            burn_router,
            widget_log_process: None,
            config,
        })
    }

    /// Initialize the management agent (lazy loading)
    pub async fn initialize_management_agent(&mut self) -> Result<()> {
        if self.management_agent.is_some() {
            return Ok(());
        }

        log::info!("Initializing Prompt Management Agent");

        let agent_config = AgentConfig::default();
        let agent = PromptManagementAgent::new(agent_config).await?;

        self.management_agent = Some(agent);
        Ok(())
    }

    /// Initialize PyO3 bridge (requires Widget-Log running)
    pub async fn initialize_pyo3_bridge(&mut self) -> Result<()> {
        if self.pyo3_bridge.is_some() {
            return Ok(());
        }

        log::info!("Initializing PyO3 bridge");

        pyo3_bridge::initialize_python()?;

        let bridge = PyO3Bridge::new(
            self.config
                .widget_log_dir
                .to_str()
                .context("Invalid widget_log_dir path")?,
        )
        .await?;

        self.pyo3_bridge = Some(bridge);
        Ok(())
    }

    /// Complete flow from user input to Anthropic response
    pub async fn handle_user_query(
        &mut self,
        user_input: &str,
        context: &ZedContext,
    ) -> Result<AgenticResponse> {
        log::info!("Processing user query: {} chars", user_input.len());

        // STAGE 1: Prompt Refinement (if agent initialized)
        let refined_prompt = if let Some(agent) = &self.management_agent {
            log::info!("Stage 1: Refining prompt with Management Agent");
            agent.refine_prompt(user_input, context).await?
        } else {
            log::info!("Stage 1: Skipping refinement (agent not initialized)");
            // Use original prompt
            RefinedPrompt {
                original: user_input.to_string(),
                refined: user_input.to_string(),
                changes: Vec::new(),
                quality_score: 0.5,
                estimated_cache_hit_improvement: 0.0,
            }
        };

        // For now, auto-approve refined prompts
        // TODO: Add user approval UI in Phase 2b
        let final_prompt = &refined_prompt.refined;

        // STAGE 2: Routing Decision (Burn-LM)
        log::info!("Stage 2: Determining routing strategy");
        let routing_decision = self.burn_router.route_prompt(final_prompt).await?;

        // STAGE 3: PyO3 Bridge to Widget-Log (if available)
        if let Some(bridge) = &self.pyo3_bridge {
            log::info!("Stage 3: Sending to Widget-Log via PyO3");

            let proxy_request = ProxyRequest {
                prompt: final_prompt.to_string(),
                cache_anchors: routing_decision.cache_anchors,
                api_key: std::env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
                optimization: routing_decision.token_optimization,
            };

            let proxy_response = bridge.send_to_proxy(proxy_request).await?;

            // Process response
            match proxy_response.cache_status {
                pyo3_bridge::CacheStatus::Hit {
                    response,
                    latency_ms,
                    similarity_score,
                } => {
                    log::info!(
                        "Cache HIT! Latency: {}ms, Similarity: {:.2}",
                        latency_ms,
                        similarity_score
                    );
                    return Ok(AgenticResponse::Cached {
                        response,
                        latency_ms,
                        similarity_score,
                        refinement_applied: !refined_prompt.changes.is_empty(),
                    });
                }
                pyo3_bridge::CacheStatus::Miss => {
                    log::info!("Cache MISS - response from API");
                    if let Some(response_text) = proxy_response.response_text {
                        return Ok(AgenticResponse::Fresh {
                            response: response_text,
                            tokens_used: proxy_response.tokens_used,
                            refinement_applied: !refined_prompt.changes.is_empty(),
                        });
                    }
                }
            }
        }

        // Fallback: return refinement info
        Ok(AgenticResponse::RefinementOnly {
            refined_prompt: refined_prompt.refined,
            changes_count: refined_prompt.changes.len(),
        })
    }

    /// Get performance statistics
    pub async fn get_stats(&self) -> Result<ManagerStats> {
        let mut stats = ManagerStats::default();

        if let Some(bridge) = &self.pyo3_bridge {
            stats.cache_stats = Some(bridge.get_cache_stats().await?);
        }

        stats.agent_initialized = self.management_agent.is_some();
        stats.bridge_initialized = self.pyo3_bridge.is_some();

        Ok(stats)
    }
}

/// Response from the integrated manager
#[derive(Debug, Clone)]
pub enum AgenticResponse {
    Cached {
        response: String,
        latency_ms: u64,
        similarity_score: f32,
        refinement_applied: bool,
    },
    Fresh {
        response: String,
        tokens_used: usize,
        refinement_applied: bool,
    },
    RefinementOnly {
        refined_prompt: String,
        changes_count: usize,
    },
}

/// Statistics from the integrated manager
#[derive(Debug, Clone, Default)]
pub struct ManagerStats {
    pub agent_initialized: bool,
    pub bridge_initialized: bool,
    pub cache_stats: Option<serde_json::Value>,
}
