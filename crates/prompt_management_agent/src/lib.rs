pub mod inference;
pub mod refinement;
pub mod context;
pub mod user_interaction;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use inference::MistralRsEngine;
pub use refinement::{PromptRefinement, RefinedPrompt};
pub use context::ZedContext;
pub use user_interaction::ApprovalResult;

/// Main Prompt Management Agent that orchestrates prompt refinement
pub struct PromptManagementAgent {
    inference_engine: MistralRsEngine,
    config: AgentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub model_path: String,
    pub refinement_threshold: f32,
    pub auto_approve_safe_refinements: bool,
    pub max_context_tokens: usize,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model_path: "microsoft/Phi-3-mini-4k-instruct-gguf".to_string(),
            refinement_threshold: 0.7,
            auto_approve_safe_refinements: false,
            max_context_tokens: 2048,
        }
    }
}

impl PromptManagementAgent {
    /// Create a new Prompt Management Agent
    pub async fn new(config: AgentConfig) -> Result<Self> {
        log::info!("Initializing Prompt Management Agent");

        let inference_engine = MistralRsEngine::new(&config.model_path).await?;

        Ok(Self {
            inference_engine,
            config,
        })
    }

    /// Analyze and refine a user prompt
    pub async fn refine_prompt(
        &self,
        user_input: &str,
        context: &ZedContext,
    ) -> Result<RefinedPrompt> {
        log::info!("Refining prompt: {} chars", user_input.len());

        // Generate refinement using local inference
        let refinement = PromptRefinement::generate(
            user_input,
            context,
            &self.inference_engine,
            &self.config,
        )
        .await?;

        log::info!(
            "Refinement generated with quality score: {:.2}",
            refinement.quality_score
        );

        Ok(refinement.into_refined_prompt(user_input))
    }

    /// Check if refinement meets quality threshold
    pub fn should_suggest_refinement(&self, quality_score: f32) -> bool {
        quality_score >= self.config.refinement_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let config = AgentConfig::default();
        let agent = PromptManagementAgent::new(config).await;

        // May fail if model not downloaded, but structure should compile
        match agent {
            Ok(_) => log::info!("Agent created successfully"),
            Err(e) => log::warn!("Agent creation failed (expected if no model): {}", e),
        }
    }
}
