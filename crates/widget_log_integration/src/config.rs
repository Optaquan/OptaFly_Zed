use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetLogConfig {
    pub widget_log_dir: PathBuf,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub auto_start: bool,
}

impl Default for WidgetLogConfig {
    fn default() -> Self {
        let widget_log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("optafly-zed")
            .join("widget-log");

        Self {
            widget_log_dir,
            proxy_host: "127.0.0.1".to_string(),
            proxy_port: 8443,
            auto_start: true,
        }
    }
}

impl WidgetLogConfig {
    /// Load configuration from file or create default
    pub async fn load_or_default(config_path: PathBuf) -> Result<Self> {
        if config_path.exists() {
            let content = tokio::fs::read_to_string(&config_path)
                .await
                .context("Failed to read config file")?;
            let config: Self = serde_json::from_str(&content)
                .context("Failed to parse config file")?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save(&config_path).await?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub async fn save(&self, config_path: &PathBuf) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create config directory")?;
        }
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        tokio::fs::write(config_path, content)
            .await
            .context("Failed to write config file")?;
        Ok(())
    }

    /// Initialize Widget-Log directory structure
    pub async fn initialize_widget_log_dir(&self) -> Result<()> {
        log::info!("Initializing Widget-Log directory: {}", self.widget_log_dir.display());

        // Create main directory
        tokio::fs::create_dir_all(&self.widget_log_dir)
            .await
            .context("Failed to create widget-log directory")?;

        // Create subdirectories
        for subdir in &["app", "certs", "logs", "external-cached"] {
            tokio::fs::create_dir_all(self.widget_log_dir.join(subdir))
                .await
                .with_context(|| format!("Failed to create {} directory", subdir))?;
        }

        // Create default project
        let default_project = self.widget_log_dir.join("OptaFly_Zed");
        tokio::fs::create_dir_all(default_project.join("contexts"))
            .await
            .context("Failed to create default project")?;

        // Create default project metadata
        let metadata = serde_json::json!({
            "name": "OptaFly_Zed",
            "description": "Default project for OptaFly_Zed caching",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "tags": ["zed", "editor", "default"]
        });
        tokio::fs::write(
            default_project.join("metadata.json"),
            serde_json::to_string_pretty(&metadata)?
        )
        .await
        .context("Failed to create project metadata")?;

        log::info!("Widget-Log directory initialized successfully");
        Ok(())
    }

    pub fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("optafly-zed")
    }

    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir().join("widget-log-config.json")
    }
}
