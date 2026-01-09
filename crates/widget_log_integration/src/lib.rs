mod config;
pub mod integrated_config;
mod lifecycle;
pub mod prompt_manager;
mod zed_config;

pub use config::WidgetLogConfig;
pub use integrated_config::IntegratedConfig;
pub use lifecycle::WidgetLogProcess;
pub use prompt_manager::{AgenticResponse, IntegratedPromptManager, ManagerStats};

use anyhow::Result;

/// Main Widget-Log integration manager
pub struct WidgetLogManager {
    config: WidgetLogConfig,
    process: Option<WidgetLogProcess>,
}

impl WidgetLogManager {
    /// Initialize Widget-Log integration
    pub async fn initialize() -> Result<Self> {
        log::info!("Initializing Widget-Log integration for OptaFly_Zed");

        let config_path = WidgetLogConfig::get_config_path();
        let config = WidgetLogConfig::load_or_default(config_path).await?;

        // Initialize directory structure
        config.initialize_widget_log_dir().await?;

        let mut process = WidgetLogProcess::new(config.proxy_port, config.proxy_host.clone());

        // Start proxy if auto_start is enabled
        if config.auto_start {
            match process.start(config.widget_log_dir.clone()).await {
                Ok(_) => {
                    log::info!("Widget-Log proxy started successfully");

                    // Configure Zed settings
                    if let Err(e) = Self::configure_zed(&config, &process).await {
                        log::warn!("Failed to configure Zed settings: {}", e);
                    }
                }
                Err(e) => {
                    log::error!("Failed to start Widget-Log proxy: {}", e);
                    log::info!("OptaFly_Zed will continue without caching");
                    return Ok(Self {
                        config,
                        process: None,
                    });
                }
            }
        }

        Ok(Self {
            config,
            process: Some(process),
        })
    }

    /// Configure Zed settings with Widget-Log proxy
    async fn configure_zed(config: &WidgetLogConfig, process: &WidgetLogProcess) -> Result<()> {
        let auth_token = zed_config::get_auth_token(&config.widget_log_dir).await?;
        let proxy_url = process.get_proxy_url();
        zed_config::configure_zed_settings(&auth_token, &proxy_url).await?;
        Ok(())
    }

    /// Start the Widget-Log proxy (if not already running)
    pub async fn start(&mut self) -> Result<()> {
        if let Some(process) = &mut self.process {
            process.start(self.config.widget_log_dir.clone()).await?;
        } else {
            let mut process =
                WidgetLogProcess::new(self.config.proxy_port, self.config.proxy_host.clone());
            process.start(self.config.widget_log_dir.clone()).await?;
            self.process = Some(process);
        }
        Ok(())
    }

    /// Stop the Widget-Log proxy
    pub fn stop(&mut self) -> Result<()> {
        if let Some(process) = &mut self.process {
            process.stop()?;
        }
        Ok(())
    }

    /// Check if Widget-Log proxy is healthy
    pub async fn health_check(&self) -> Result<bool> {
        if let Some(process) = &self.process {
            process.health_check().await
        } else {
            Ok(false)
        }
    }

    /// Get the proxy URL for direct access
    pub fn get_proxy_url(&self) -> Option<String> {
        self.process.as_ref().map(|p| p.get_proxy_url())
    }

    /// Get Widget-Log statistics
    pub async fn get_stats(&self) -> Result<serde_json::Value> {
        if let Some(process) = &self.process {
            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(2))
                .build()?;

            let auth_token = zed_config::get_auth_token(&self.config.widget_log_dir).await?;
            let url = format!(
                "{}:/{}:{}/stats",
                "https", self.config.proxy_host, self.config.proxy_port
            );

            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", auth_token))
                .send()
                .await?;

            let stats = response.json().await?;
            Ok(stats)
        } else {
            Ok(serde_json::json!({
                "error": "Widget-Log proxy not running"
            }))
        }
    }
}

impl Drop for WidgetLogManager {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Initialize Widget-Log for OptaFly_Zed
/// This should be called during Zed application startup
pub async fn initialize() -> Result<WidgetLogManager> {
    WidgetLogManager::initialize().await
}
