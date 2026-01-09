use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// Telemetry event types for comprehensive monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum TelemetryEvent {
    /// Prompt processing started
    PromptStart {
        prompt_id: String,
        prompt_length: usize,
        has_cache_anchors: bool,
        timestamp: u64,
    },

    /// Prompt processing completed
    PromptEnd {
        prompt_id: String,
        duration_ms: u64,
        success: bool,
        error_message: Option<String>,
        timestamp: u64,
    },

    /// Cache hit occurred
    CacheHit {
        prompt_id: String,
        similarity_score: f32,
        latency_ms: u64,
        tokens_saved: usize,
        timestamp: u64,
    },

    /// Cache miss occurred
    CacheMiss {
        prompt_id: String,
        reason: CacheMissReason,
        timestamp: u64,
    },

    /// Model inference started
    ModelInferenceStart {
        prompt_id: String,
        model_name: String,
        prompt_tokens: usize,
        timestamp: u64,
    },

    /// Model inference completed
    ModelInferenceEnd {
        prompt_id: String,
        duration_ms: u64,
        tokens_generated: usize,
        tokens_per_second: f32,
        timestamp: u64,
    },

    /// PyO3 bridge initialization
    BridgeInitialized {
        widget_log_path: String,
        module_cached: bool,
        duration_ms: u64,
        timestamp: u64,
    },

    /// PyO3 bridge error
    BridgeError {
        operation: String,
        error_type: String,
        error_message: String,
        prompt_id: Option<String>,
        timestamp: u64,
    },

    /// Burn-LM router decision
    BurnRouterDecision {
        prompt_id: String,
        strategy: String,
        cache_anchors_count: usize,
        estimated_tokens: usize,
        timestamp: u64,
    },

    /// Prompt management agent refinement
    PromptRefinement {
        prompt_id: String,
        original_length: usize,
        refined_length: usize,
        quality_score: f32,
        changes_count: usize,
        timestamp: u64,
    },

    /// Health check performed
    HealthCheck {
        status: HealthStatus,
        details: Option<String>,
        timestamp: u64,
    },

    /// Cache statistics snapshot
    CacheStats {
        hit_rate: f32,
        total_hits: u64,
        total_misses: u64,
        total_requests: u64,
        avg_latency_ms: f32,
        cache_size_bytes: u64,
        timestamp: u64,
    },

    /// Custom event for extensibility
    Custom {
        name: String,
        data: HashMap<String, serde_json::Value>,
        timestamp: u64,
    },
}

/// Reason for cache miss
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheMissReason {
    NoSimilarPrompt,
    SimilarityBelowThreshold,
    CacheEmpty,
    CacheDisabled,
    Error(String),
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl TelemetryEvent {
    /// Get current Unix timestamp in milliseconds
    pub fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Create a PromptStart event
    pub fn prompt_start(prompt_id: String, prompt_length: usize, has_cache_anchors: bool) -> Self {
        Self::PromptStart {
            prompt_id,
            prompt_length,
            has_cache_anchors,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a PromptEnd event
    pub fn prompt_end(
        prompt_id: String,
        duration_ms: u64,
        success: bool,
        error_message: Option<String>,
    ) -> Self {
        Self::PromptEnd {
            prompt_id,
            duration_ms,
            success,
            error_message,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a CacheHit event
    pub fn cache_hit(
        prompt_id: String,
        similarity_score: f32,
        latency_ms: u64,
        tokens_saved: usize,
    ) -> Self {
        Self::CacheHit {
            prompt_id,
            similarity_score,
            latency_ms,
            tokens_saved,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a CacheMiss event
    pub fn cache_miss(prompt_id: String, reason: CacheMissReason) -> Self {
        Self::CacheMiss {
            prompt_id,
            reason,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a BridgeError event
    pub fn bridge_error(
        operation: impl Into<String>,
        error_type: impl Into<String>,
        error_message: impl Into<String>,
        prompt_id: Option<String>,
    ) -> Self {
        Self::BridgeError {
            operation: operation.into(),
            error_type: error_type.into(),
            error_message: error_message.into(),
            prompt_id,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a BridgeInitialized event
    pub fn bridge_initialized(
        widget_log_path: String,
        module_cached: bool,
        duration_ms: u64,
    ) -> Self {
        Self::BridgeInitialized {
            widget_log_path,
            module_cached,
            duration_ms,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a HealthCheck event
    pub fn health_check(status: HealthStatus, details: Option<String>) -> Self {
        Self::HealthCheck {
            status,
            details,
            timestamp: Self::now_ms(),
        }
    }

    /// Create a Custom event
    pub fn custom(name: impl Into<String>, data: HashMap<String, serde_json::Value>) -> Self {
        Self::Custom {
            name: name.into(),
            data,
            timestamp: Self::now_ms(),
        }
    }

    /// Get the event's timestamp
    pub fn timestamp(&self) -> u64 {
        match self {
            Self::PromptStart { timestamp, .. }
            | Self::PromptEnd { timestamp, .. }
            | Self::CacheHit { timestamp, .. }
            | Self::CacheMiss { timestamp, .. }
            | Self::ModelInferenceStart { timestamp, .. }
            | Self::ModelInferenceEnd { timestamp, .. }
            | Self::BridgeInitialized { timestamp, .. }
            | Self::BridgeError { timestamp, .. }
            | Self::BurnRouterDecision { timestamp, .. }
            | Self::PromptRefinement { timestamp, .. }
            | Self::HealthCheck { timestamp, .. }
            | Self::CacheStats { timestamp, .. }
            | Self::Custom { timestamp, .. } => *timestamp,
        }
    }

    /// Get the prompt_id if available
    pub fn prompt_id(&self) -> Option<&str> {
        match self {
            Self::PromptStart { prompt_id, .. }
            | Self::PromptEnd { prompt_id, .. }
            | Self::CacheHit { prompt_id, .. }
            | Self::CacheMiss { prompt_id, .. }
            | Self::ModelInferenceStart { prompt_id, .. }
            | Self::ModelInferenceEnd { prompt_id, .. }
            | Self::BurnRouterDecision { prompt_id, .. }
            | Self::PromptRefinement { prompt_id, .. } => Some(prompt_id),
            Self::BridgeError { prompt_id, .. } => prompt_id.as_deref(),
            _ => None,
        }
    }

    /// Get a human-readable event name
    pub fn event_name(&self) -> &'static str {
        match self {
            Self::PromptStart { .. } => "prompt_start",
            Self::PromptEnd { .. } => "prompt_end",
            Self::CacheHit { .. } => "cache_hit",
            Self::CacheMiss { .. } => "cache_miss",
            Self::ModelInferenceStart { .. } => "model_inference_start",
            Self::ModelInferenceEnd { .. } => "model_inference_end",
            Self::BridgeInitialized { .. } => "bridge_initialized",
            Self::BridgeError { .. } => "bridge_error",
            Self::BurnRouterDecision { .. } => "burn_router_decision",
            Self::PromptRefinement { .. } => "prompt_refinement",
            Self::HealthCheck { .. } => "health_check",
            Self::CacheStats { .. } => "cache_stats",
            Self::Custom { .. } => "custom",
        }
    }

    /// Get the severity level of this event
    pub fn level(&self) -> &'static str {
        match self {
            Self::BridgeError { .. } => "error",
            Self::CacheMiss {
                reason: CacheMissReason::Error(_),
                ..
            } => "error",
            Self::HealthCheck {
                status: HealthStatus::Unhealthy,
                ..
            } => "error",
            Self::HealthCheck {
                status: HealthStatus::Degraded,
                ..
            } => "warn",
            Self::PromptEnd { success: false, .. } => "error",
            Self::CacheMiss { .. } => "debug",
            Self::PromptStart { .. }
            | Self::ModelInferenceStart { .. }
            | Self::BurnRouterDecision { .. } => "debug",
            _ => "info",
        }
    }

    /// Get correlation ID (alias for prompt_id for tracing compatibility)
    pub fn correlation_id(&self) -> Option<&str> {
        self.prompt_id()
    }

    /// Get trace ID (same as correlation_id, for OpenTelemetry compatibility)
    pub fn trace_id(&self) -> Option<&str> {
        self.prompt_id()
    }

    /// Check if this event should be sampled based on config
    pub fn should_sample(&self, sample_rate: f32) -> bool {
        if sample_rate >= 1.0 {
            return true;
        }
        if sample_rate <= 0.0 {
            return false;
        }

        // Always include errors and critical events
        match self.level() {
            "error" | "warn" => true,
            _ => {
                // Use prompt_id-based deterministic sampling for consistency
                if let Some(prompt_id) = self.prompt_id() {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    prompt_id.hash(&mut hasher);
                    let hash_val = hasher.finish();
                    let threshold = (sample_rate as f64 * u64::MAX as f64) as u64;
                    hash_val <= threshold
                } else {
                    // Fallback to timestamp-based sampling for events without prompt_id
                    let sample_threshold = (sample_rate * 100.0) as u64;
                    (self.timestamp() % 100) < sample_threshold
                }
            }
        }
    }
}

/// Telemetry collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Enable telemetry collection
    pub enabled: bool,

    /// Batch size before sending to Python
    pub batch_size: usize,

    /// Maximum queue size before dropping events
    pub max_queue_size: usize,

    /// Enable debug logging of events
    pub debug_logging: bool,

    /// Sample rate (0.0-1.0) for high-volume events
    pub sample_rate: f32,

    /// Per-event-type sampling overrides
    pub event_sampling: HashMap<String, f32>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        let mut event_sampling = HashMap::new();
        event_sampling.insert("prompt_start".to_string(), 0.1);
        event_sampling.insert("cache_miss".to_string(), 0.5);
        event_sampling.insert("burn_router_decision".to_string(), 0.2);

        Self {
            enabled: true,
            batch_size: 10,
            max_queue_size: 1000,
            debug_logging: false,
            sample_rate: 1.0,
            event_sampling,
        }
    }
}

impl TelemetryConfig {
    /// Get the effective sample rate for a given event
    pub fn sample_rate_for_event(&self, event: &TelemetryEvent) -> f32 {
        self.event_sampling
            .get(event.event_name())
            .copied()
            .unwrap_or(self.sample_rate)
    }

    /// Check if an event should be collected
    pub fn should_collect(&self, event: &TelemetryEvent) -> bool {
        if !self.enabled {
            return false;
        }

        let sample_rate = self.sample_rate_for_event(event);
        event.should_sample(sample_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_event_creation() {
        let event = TelemetryEvent::prompt_start("test-123".to_string(), 100, true);
        assert_eq!(event.event_name(), "prompt_start");
        assert_eq!(event.prompt_id(), Some("test-123"));
        assert_eq!(event.level(), "debug");
    }

    #[test]
    fn test_telemetry_event_serialization() {
        let event = TelemetryEvent::cache_hit("test-456".to_string(), 0.92, 50, 1000);
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("cache_hit"));
        assert!(json.contains("test-456"));
    }

    #[test]
    fn test_cache_miss_reason() {
        let event = TelemetryEvent::cache_miss(
            "test-789".to_string(),
            CacheMissReason::SimilarityBelowThreshold,
        );
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("similarity_below_threshold"));
    }

    #[test]
    fn test_severity_levels() {
        let error = TelemetryEvent::bridge_error("test_op", "TestError", "test message", None);
        assert_eq!(error.level(), "error");

        let healthy = TelemetryEvent::health_check(HealthStatus::Healthy, None);
        assert_eq!(healthy.level(), "info");

        let degraded = TelemetryEvent::health_check(HealthStatus::Degraded, None);
        assert_eq!(degraded.level(), "warn");
    }

    #[test]
    fn test_correlation_ids() {
        let event = TelemetryEvent::prompt_start("trace-123".to_string(), 100, true);
        assert_eq!(event.correlation_id(), Some("trace-123"));
        assert_eq!(event.trace_id(), Some("trace-123"));
    }

    #[test]
    fn test_sampling() {
        let event = TelemetryEvent::prompt_start("test-sample".to_string(), 100, false);

        assert!(event.should_sample(1.0));
        assert!(!event.should_sample(0.0));

        let error = TelemetryEvent::bridge_error("test", "error", "message", None);
        assert!(error.should_sample(0.01));
    }

    #[test]
    fn test_config_per_event_sampling() {
        let config = TelemetryConfig::default();
        let start_event = TelemetryEvent::prompt_start("test".to_string(), 100, false);
        let error_event = TelemetryEvent::bridge_error("op", "type", "msg", None);

        assert_eq!(config.sample_rate_for_event(&start_event), 0.1);
        assert_eq!(config.sample_rate_for_event(&error_event), 1.0);
    }
}
