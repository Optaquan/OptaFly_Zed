use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

/// Integrated configuration for all OptaFly_Zed enhancement layers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedConfig {
    pub widget_log: WidgetLogSection,
    pub prompt_management: PromptManagementSection,
    pub burn_router: BurnRouterSection,
    pub pyo3_bridge: PyO3BridgeSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetLogSection {
    pub enabled: bool,
    pub auto_start: bool,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub widget_log_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptManagementSection {
    pub enabled: bool,
    pub model: String,
    pub refinement_threshold: f32,
    pub auto_approve_safe_refinements: bool,
    pub max_context_tokens: usize,
    pub include_active_file: bool,
    pub include_selection: bool,
    pub include_project_structure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnRouterSection {
    pub enabled: bool,
    pub cache_anchor_optimization: bool,
    pub token_optimization: bool,
    pub routing_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyO3BridgeSection {
    pub enabled: bool,
    pub gil_optimization: bool,
    pub zero_copy_transfers: bool,
    pub connection_pool_size: usize,
}

impl Default for IntegratedConfig {
    fn default() -> Self {
        let widget_log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("optafly-zed")
            .join("widget-log");

        Self {
            widget_log: WidgetLogSection {
                enabled: true,
                auto_start: true,
                proxy_host: "127.0.0.1".to_string(),
                proxy_port: 8443,
                widget_log_dir,
            },
            prompt_management: PromptManagementSection {
                enabled: true,
                model: "phi-3-mini-q4".to_string(),
                refinement_threshold: 0.7,
                auto_approve_safe_refinements: false,
                max_context_tokens: 2048,
                include_active_file: true,
                include_selection: true,
                include_project_structure: true,
            },
            burn_router: BurnRouterSection {
                enabled: true,
                cache_anchor_optimization: true,
                token_optimization: true,
                routing_strategy: "auto".to_string(),
            },
            pyo3_bridge: PyO3BridgeSection {
                enabled: true,
                gil_optimization: true,
                zero_copy_transfers: true,
                connection_pool_size: 4,
            },
        }
    }
}

impl IntegratedConfig {
    /// Load configuration from file or create default
    pub async fn load_or_default() -> Result<Self> {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Self = serde_json::from_str(&content)?;
            log::info!("Loaded configuration from {:?}", config_path);
            Ok(config)
        } else {
            let config = Self::default();
            config.save().await?;
            log::info!("Created default configuration at {:?}", config_path);
            Ok(config)
        }
    }

    /// Save configuration to file
    pub async fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;

        log::info!("Saved configuration to {:?}", config_path);
        Ok(())
    }

    /// Get the configuration file path
    pub fn get_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("optafly-zed")
            .join("integrated-config.json")
    }
}
