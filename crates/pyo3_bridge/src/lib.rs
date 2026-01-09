pub mod bridge;
pub mod conversions;
pub mod types;

use anyhow::Result;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub use bridge::PyO3Bridge;
pub use types::{ProxyRequest, ProxyResponse, CacheStatus};

/// Initialize PyO3 and prepare Python interpreter
pub fn initialize_python() -> Result<()> {
    pyo3::prepare_freethreaded_python();
    log::info!("PyO3 initialized successfully");
    Ok(())
}

/// Global Python bridge instance
static BRIDGE_INSTANCE: once_cell::sync::Lazy<Arc<Mutex<Option<PyO3Bridge>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

/// Initialize the global bridge instance
pub async fn initialize_bridge(widget_log_path: &str) -> Result<()> {
    let bridge = PyO3Bridge::new(widget_log_path).await?;
    let mut instance = BRIDGE_INSTANCE.lock().await;
    *instance = Some(bridge);
    Ok(())
}

/// Get reference to the global bridge
pub async fn get_bridge() -> Result<Arc<Mutex<Option<PyO3Bridge>>>> {
    Ok(BRIDGE_INSTANCE.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_initialization() {
        let result = initialize_python();
        assert!(result.is_ok());
    }
}
