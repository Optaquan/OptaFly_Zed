use serde::{Deserialize, Serialize};

/// Request sent to Widget-Log proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRequest {
    pub prompt: String,
    pub cache_anchors: Vec<CacheAnchor>,
    pub api_key: String,
    pub optimization: TokenOptimization,
}

/// Response from Widget-Log proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyResponse {
    pub cache_status: CacheStatus,
    pub response_text: Option<String>,
    pub tokens_used: usize,
    pub latency_ms: u64,
}

/// Cache hit or miss status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStatus {
    Hit {
        response: String,
        latency_ms: u64,
        similarity_score: f32,
    },
    Miss,
}

/// Cache anchor for Anthropic prompt caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAnchor {
    pub position: usize,
    pub content: String,
    pub priority: f32,
    pub estimated_tokens: usize,
}

/// Token optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenOptimization {
    pub remove_redundancy: bool,
    pub compress_context: bool,
    pub split_into_chunks: bool,
}

impl Default for TokenOptimization {
    fn default() -> Self {
        Self {
            remove_redundancy: true,
            compress_context: false,
            split_into_chunks: false,
        }
    }
}
