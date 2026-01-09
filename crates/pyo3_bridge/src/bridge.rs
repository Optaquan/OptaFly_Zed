use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::telemetry::TelemetryEvent;
use crate::types::{CacheStatus, ProxyRequest, ProxyResponse};

/// High-performance Rust ↔ Python bridge for Widget-Log
pub struct PyO3Bridge {
    widget_log_path: PathBuf,
    initialized: Arc<Mutex<bool>>,
    py_module: Arc<Mutex<Option<Py<PyModule>>>>,
}

impl PyO3Bridge {
    /// Create a new PyO3 bridge
    pub async fn new(widget_log_path: &str) -> Result<Self> {
        log::info!("Initializing PyO3 bridge for Widget-Log");

        let path = PathBuf::from(widget_log_path);

        // Verify Widget-Log directory exists
        if !path.exists() {
            anyhow::bail!("Widget-Log directory not found: {}", path.display());
        }

        Ok(Self {
            widget_log_path: path,
            initialized: Arc::new(Mutex::new(false)),
            py_module: Arc::new(Mutex::new(None)),
        })
    }

    /// Initialize Widget-Log Python module (idempotent)
    pub async fn initialize(&self) -> Result<()> {
        let mut init = self.initialized.lock().await;
        if *init {
            log::debug!("Widget-Log already initialized, skipping");
            return Ok(());
        }

        let widget_log_path = self.widget_log_path.clone();
        let py_module_cache = self.py_module.clone();
        let start_time = std::time::Instant::now();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                log::info!("Importing Widget-Log Python module");

                // Add Widget-Log to Python path (idempotent)
                let sys = py.import("sys").context("Failed to import sys module")?;
                let sys_path: &PyList = sys
                    .getattr("path")?
                    .downcast()
                    .map_err(|e| anyhow::anyhow!("sys.path is not a list: {}", e))?;

                let path_str = widget_log_path
                    .to_str()
                    .context("Invalid UTF-8 in widget_log_path")?;

                // Convert path to Python string
                let path_py = PyString::new(py, path_str);

                // Idempotent insert at front (checks for duplicates)
                if !sys_path.contains(path_py)? {
                    sys_path.insert(0, path_py)?;
                    log::debug!("Added {} to sys.path", path_str);
                } else {
                    log::debug!("Path {} already in sys.path", path_str);
                }

                // Import Widget-Log module
                let widget_log = PyModule::import(py, "widget_log_proxy").with_context(|| {
                    format!(
                        "Failed to import widget_log_proxy from {}",
                        widget_log_path.display()
                    )
                })?;

                // Call initialize function
                let config_path = widget_log_path.join("config.yaml");
                let config_str = config_path
                    .to_str()
                    .context("Invalid UTF-8 in config path")?;

                let init_result = widget_log
                    .getattr("initialize")?
                    .call1((config_str,))?
                    .extract::<String>()?;

                log::debug!("Widget-Log initialize result: {}", init_result);

                // Parse result to check success
                let result: serde_json::Value = serde_json::from_str(&init_result)
                    .context("Failed to parse initialization result")?;

                if !result["success"].as_bool().unwrap_or(false) {
                    anyhow::bail!(
                        "Widget-Log initialization failed: {}",
                        result["message"].as_str().unwrap_or("Unknown error")
                    );
                }

                // Cache the module for future use
                {
                    let mut cached = py_module_cache.blocking_lock();
                    *cached = Some(widget_log.into_py(py));
                    log::info!("Widget-Log module cached successfully");
                }

                Ok::<(), anyhow::Error>(())
            })
        })
        .await
        .context("Failed to spawn blocking task")??;

        *init = true;
        let duration_ms = start_time.elapsed().as_millis() as u64;
        log::info!(
            "Widget-Log Python module initialized successfully in {}ms",
            duration_ms
        );

        // Send initialization telemetry (fire-and-forget)
        let _ = self
            .send_telemetry(TelemetryEvent::bridge_initialized(
                self.widget_log_path.display().to_string(),
                true,
                duration_ms,
            ))
            .await;

        Ok(())
    }

    /// Send refined prompt to Widget-Log proxy
    pub async fn send_to_proxy(&self, request: ProxyRequest) -> Result<ProxyResponse> {
        // Ensure initialized
        self.initialize().await?;

        log::debug!("Sending request to Widget-Log proxy via PyO3");

        let py_module_cache = self.py_module.clone();
        let response = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| Self::call_widget_log_python(py, &py_module_cache, request))
        })
        .await
        .context("Failed to spawn blocking task")?
        .context("Python call failed")?;

        Ok(response)
    }

    /// Internal Python call (runs with GIL)
    fn call_widget_log_python(
        py: Python,
        py_module_cache: &Arc<Mutex<Option<Py<PyModule>>>>,
        request: ProxyRequest,
    ) -> Result<ProxyResponse> {
        // Retrieve cached module — clone handle and keep it alive
        let module_handle: Option<Py<PyModule>> = {
            let cached = py_module_cache.blocking_lock();
            cached.as_ref().map(|m| m.clone_ref(py))
        };

        let widget_log: &PyModule = match &module_handle {
            Some(handle) => handle.as_ref(py),
            None => {
                log::warn!("Module not cached, importing directly (performance impact)");
                PyModule::import(py, "widget_log_proxy")
                    .context("Failed to import widget_log_proxy")?
            }
        };

        // Serialize request to JSON
        let request_json =
            serde_json::to_string(&request).context("Failed to serialize ProxyRequest")?;
        let request_py = PyString::new(py, &request_json);

        log::debug!("Calling widget_log_proxy.process_prompt()");

        // Call process_prompt function
        let result_py = widget_log
            .getattr("process_prompt")
            .context("widget_log_proxy.process_prompt not found")?
            .call1((request_py,))
            .context("process_prompt() call failed")?;

        // Extract result string
        let result_str: String = result_py
            .extract()
            .context("Failed to extract string from Python result")?;

        log::debug!(
            "Received response from Widget-Log: {} bytes",
            result_str.len()
        );

        // Parse JSON response
        let response_json: serde_json::Value =
            serde_json::from_str(&result_str).context("Failed to parse response JSON")?;

        // Check for errors
        if let Some(error) = response_json.get("error") {
            anyhow::bail!("Widget-Log error: {}", error.as_str().unwrap_or("Unknown"));
        }

        // Convert to ProxyResponse
        let cache_hit = response_json["cache_hit"].as_bool().unwrap_or(false);

        let cache_status = if cache_hit {
            CacheStatus::Hit {
                response: response_json["response"].as_str().unwrap_or("").to_string(),
                latency_ms: response_json["latency_ms"].as_u64().unwrap_or(0),
                similarity_score: response_json["similarity_score"].as_f64().unwrap_or(0.0) as f32,
            }
        } else {
            CacheStatus::Miss
        };

        let response = ProxyResponse {
            cache_status,
            response_text: response_json["response_text"]
                .as_str()
                .map(|s| s.to_string()),
            tokens_used: response_json["tokens_used"].as_u64().unwrap_or(0) as usize,
            latency_ms: response_json["latency_ms"].as_u64().unwrap_or(0),
        };

        Ok(response)
    }

    /// Get cache statistics from Widget-Log
    pub async fn get_cache_stats(&self) -> Result<serde_json::Value> {
        self.initialize().await?;

        let py_module_cache = self.py_module.clone();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                // Retrieve cached module — clone handle and keep it alive
                let module_handle: Option<Py<PyModule>> = {
                    let cached = py_module_cache.blocking_lock();
                    cached.as_ref().map(|m| m.clone_ref(py))
                };

                let widget_log: &PyModule = match &module_handle {
                    Some(handle) => handle.as_ref(py),
                    None => {
                        log::warn!("Module not cached, falling back to direct import");
                        PyModule::import(py, "widget_log_proxy")
                            .context("Failed to import widget_log_proxy")?
                    }
                };

                log::debug!("Calling widget_log_proxy.get_cache_stats()");

                let stats_py = widget_log.getattr("get_cache_stats")?.call0()?;

                let stats_str: String = stats_py.extract()?;
                let stats: serde_json::Value =
                    serde_json::from_str(&stats_str).context("Failed to parse cache stats")?;

                Ok(stats)
            })
        })
        .await
        .context("Failed to spawn blocking task")?
    }

    /// Check Widget-Log health
    pub async fn health_check(&self) -> Result<serde_json::Value> {
        // Don't require initialization for health check
        let py_module_cache = self.py_module.clone();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                // Retrieve cached module — clone handle and keep it alive
                let module_handle: Option<Py<PyModule>> = {
                    let cached = py_module_cache.blocking_lock();
                    cached.as_ref().map(|m| m.clone_ref(py))
                };

                let widget_log: &PyModule = match &module_handle {
                    Some(handle) => handle.as_ref(py),
                    None => {
                        // Try import for health check
                        match PyModule::import(py, "widget_log_proxy") {
                            Ok(module) => module,
                            Err(_) => {
                                // Return unhealthy if can't import
                                return Ok(serde_json::json!({
                                    "status": "unhealthy",
                                    "details": "Module not importable"
                                }));
                            }
                        }
                    }
                };

                let health_py = widget_log.getattr("health_check")?.call0()?;

                let health_str: String = health_py.extract()?;
                let health: serde_json::Value = serde_json::from_str(&health_str)
                    .context("Failed to parse health check result")?;

                Ok(health)
            })
        })
        .await
        .context("Failed to spawn blocking task")?
    }

    /// Send a telemetry event to Widget-Log (fire-and-forget, non-blocking)
    pub async fn send_telemetry(&self, event: TelemetryEvent) -> Result<()> {
        // Quick guard: skip if not initialized (telemetry is best-effort)
        if !*self.initialized.lock().await {
            log::debug!(
                "Bridge not initialized, dropping telemetry event: {:?}",
                event.event_name()
            );
            return Ok(());
        }

        let py_module_cache = self.py_module.clone();

        // Serialize early — cheap and allows early failure detection
        let event_json = match serde_json::to_string(&event) {
            Ok(json) => json,
            Err(e) => {
                log::warn!("Failed to serialize telemetry event: {}", e);
                return Ok(());
            }
        };

        // Fire-and-forget: spawn a lightweight task
        tokio::task::spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                Python::with_gil(|py| {
                    let module_handle: Option<Py<PyModule>> = {
                        let cached = py_module_cache.blocking_lock();
                        cached.as_ref().map(|m| m.clone_ref(py))
                    };

                    let widget_log: &PyModule = match &module_handle {
                        Some(handle) => handle.as_ref(py),
                        None => {
                            // Fallback import — rare, but safe
                            match PyModule::import(py, "widget_log_proxy") {
                                Ok(m) => m,
                                Err(_) => return,
                            }
                        }
                    };

                    let event_py_obj = PyString::new(py, &event_json);

                    // Call Widget-Log's log_event function
                    if let Err(e) = widget_log
                        .getattr("log_event")
                        .and_then(|f| f.call1((event_py_obj,)))
                    {
                        log::debug!("Failed to send telemetry (non-critical): {}", e);
                    }
                })
            })
            .await;

            if let Err(e) = result {
                log::debug!("Telemetry task panicked or failed: {}", e);
            }
        });

        Ok(())
    }

    /// Send telemetry and wait for completion (blocking version for critical events)
    pub async fn send_telemetry_critical(&self, event: TelemetryEvent) -> Result<()> {
        if !*self.initialized.lock().await {
            log::debug!(
                "Bridge not initialized, dropping critical telemetry event: {:?}",
                event.event_name()
            );
            return Ok(());
        }

        let py_module_cache = self.py_module.clone();
        let event_json =
            serde_json::to_string(&event).context("Failed to serialize telemetry event")?;

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                let module_handle: Option<Py<PyModule>> = {
                    let cached = py_module_cache.blocking_lock();
                    cached.as_ref().map(|m| m.clone_ref(py))
                };

                let widget_log: &PyModule = match &module_handle {
                    Some(handle) => handle.as_ref(py),
                    None => PyModule::import(py, "widget_log_proxy")
                        .context("Failed to import widget_log_proxy")?,
                };

                let event_py_obj = PyString::new(py, &event_json);

                widget_log
                    .getattr("log_event")
                    .context("widget_log_proxy.log_event not found")?
                    .call1((event_py_obj,))
                    .context("log_event() call failed")?;

                Ok::<(), anyhow::Error>(())
            })
        })
        .await
        .context("Failed to spawn blocking task")??;

        Ok(())
    }

    /// Send a critical error event (convenience wrapper)
    pub async fn send_critical_error(
        &self,
        operation: impl Into<String>,
        message: impl Into<String>,
        prompt_id: Option<String>,
    ) {
        let _ = self
            .send_telemetry_critical(TelemetryEvent::bridge_error(
                operation, "critical", message, prompt_id,
            ))
            .await;
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
        // This will fail without actual widget-log directory
        let result = PyO3Bridge::new("/tmp/test-widget-log").await;
        match result {
            Ok(_) => log::info!("Bridge created (unexpected with mock path)"),
            Err(e) => log::info!("Bridge creation failed as expected: {}", e),
        }
    }
}
