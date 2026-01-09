use pyo3_bridge::types::{CacheAnchor, TokenOptimization};
use serde::{Deserialize, Serialize};

/// Routing decision for a prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub use_cache: bool,
    pub cache_anchors: Vec<CacheAnchor>,
    pub api_key_selection: ApiKeyConfig,
    pub token_optimization: TokenOptimization,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiKeyConfig {
    UseDefault,
    UserSpecified(String),
}

impl RoutingDecision {
    /// Create a simple routing decision (fallback)
    pub fn simple(prompt: &str) -> Self {
        Self {
            use_cache: true,
            cache_anchors: Vec::new(),
            api_key_selection: ApiKeyConfig::UseDefault,
            token_optimization: TokenOptimization::default(),
        }
    }
}
