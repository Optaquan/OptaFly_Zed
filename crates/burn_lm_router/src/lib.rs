pub mod cache_anchor;
pub mod network;
pub mod routing;
pub mod token_optimizer;

use anyhow::Result;

pub use cache_anchor::CacheAnchorAnalyzer;
pub use routing::{ApiKeyConfig, RoutingDecision};
pub use token_optimizer::TokenOptimizer;

/// Burn-LM Router for intelligent request routing
/// Currently uses heuristic-based routing (Phase 2a)
/// Will integrate neural networks in Phase 2c with burn-neural-networks feature
pub struct BurnLmRouter {
    cache_anchor_analyzer: CacheAnchorAnalyzer,
    token_optimizer: TokenOptimizer,
}

impl BurnLmRouter {
    /// Create a new Burn-LM router
    pub fn new() -> Self {
        log::info!("Initializing Burn-LM router (heuristic mode)");

        Self {
            cache_anchor_analyzer: CacheAnchorAnalyzer::new(),
            token_optimizer: TokenOptimizer::new(),
        }
    }

    /// Route a refined prompt through optimal path
    pub async fn route_prompt(&self, refined_prompt: &str) -> Result<RoutingDecision> {
        log::debug!("Routing prompt: {} chars", refined_prompt.len());

        // Extract optimal cache anchors
        let cache_anchors = self
            .cache_anchor_analyzer
            .extract_anchors(refined_prompt)
            .await?;

        // Optimize token usage
        let token_optimization = self.token_optimizer.optimize(refined_prompt).await?;

        // Determine if we should use cache
        let use_cache = self.should_use_cache(refined_prompt);

        Ok(RoutingDecision {
            use_cache,
            cache_anchors,
            api_key_selection: ApiKeyConfig::UseDefault,
            token_optimization,
        })
    }

    fn should_use_cache(&self, prompt: &str) -> bool {
        // Use cache if prompt is sufficiently detailed
        prompt.len() > 50 && prompt.contains(|c: char| c.is_alphanumeric())
    }
}

impl Default for BurnLmRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router_creation() {
        let router = BurnLmRouter::new();
        let decision = router.route_prompt("Test prompt for routing").await;
        assert!(decision.is_ok());
    }
}
