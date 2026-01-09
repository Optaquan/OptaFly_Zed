use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::path::PathBuf;

use crate::types::{ProxyRequest, ProxyResponse};

/// High-performance Rust ↔ Python bridge for Widget-Log
pub struct PyO3Bridge {
    widget_log_path: PathBuf,
}

impl PyO3Bridge {
    /// Create a new PyO3 bridge
    pub async fn new(widget_log_path: &str) -> Result<Self> {
        log::info!("Initializing PyO3 bridge for Widget-Log");

        let path = PathBuf::from(widget_log_path);

        // Verify Widget-Log directory exists
        if !path.exists() {
            anyhow::bail!(
                "Widget-Log directory not found: {}",
                path.display()
            );
        }

        Ok(Self {
            widget_log_path: path,
        })
    }

    /// Send refined prompt to Widget-Log proxy
    pub async fn send_to_proxy(&self, request: ProxyRequest) -> Result<ProxyResponse> {
        log::debug!("Sending request to Widget-Log proxy via PyO3");

        // Use tokio::task::spawn_blocking for Python GIL operations
        let widget_log_path = self.widget_log_path.clone();
        let response = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                Self::call_proxy_python(py, &widget_log_path, request)
            })
        })
        .await
        .context("Failed to spawn blocking task")?
        .context("Python call failed")?;

        Ok(response)
    }

    /// Internal Python call (runs with GIL)
    fn call_proxy_python(
        py: Python,
        widget_log_path: &PathBuf,
        request: ProxyRequest,
    ) -> Result<ProxyResponse> {
        // Add Widget-Log to Python path
        let sys = py.import("sys")?;
        let sys_path = sys.getattr("path")?;
        sys_path.call_method1("append", (widget_log_path.to_str().unwrap(),))?;

        // Import Widget-Log module (placeholder - actual module will be in widget-log/)
        // For now, we create a mock response to demonstrate the structure
        log::info!("Python bridge: processing request for prompt with {} chars", request.prompt.len());

        // TODO: Actual Widget-Log Python module call will be:
        // let widget_log = PyModule::import(py, "widget_log_proxy")?;
        // let result = widget_log.call_method1("process_request", (request_dict,))?;

        // Mock response for Phase 2a
        let response = ProxyResponse {
            cache_status: crate::types::CacheStatus::Miss,
            response_text: None,
            tokens_used: 0,
            latency_ms: 0,
        };

        Ok(response)
    }

    /// Call Widget-Log Python function with zero-copy where possible
    pub async fn call_widget_log_function(
        &self,
        function_name: &str,
        args: Vec<String>,
    ) -> Result<String> {
        let widget_log_path = self.widget_log_path.clone();
        let function = function_name.to_string();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                log::debug!("Calling Widget-Log function: {}", function);

                // Add to Python path
                let sys = py.import("sys")?;
                let sys_path = sys.getattr("path")?;
                sys_path.call_method1("append", (widget_log_path.to_str().unwrap(),))?;

                // Mock implementation
                Ok(format!("Called {} with {} args", function, args.len()))
            })
        })
        .await?
    }

    /// Get cache statistics from Widget-Log
    pub async fn get_cache_stats(&self) -> Result<serde_json::Value> {
        let widget_log_path = self.widget_log_path.clone();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                log::debug!("Fetching cache statistics");

                // Mock statistics
                Ok(serde_json::json!({
                    "cache_hits": 0,
                    "cache_misses": 0,
                    "hit_rate": 0.0,
                    "total_requests": 0
                }))
            })
        })
        .await?
    }
}

// Implement Send + Sync for PyO3Bridge
unsafe impl Send for PyO3Bridge {}
unsafe impl Sync for PyO3Bridge {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bridge_creation() {
        // This will fail without actual widget-log directory, but tests structure
        let result = PyO3Bridge::new("/tmp/test-widget-log").await;
        match result {
            Ok(_) => log::info!("Bridge created (unexpected with mock path)"),
            Err(e) => log::info!("Bridge creation failed as expected: {}", e),
        }
    }
}
