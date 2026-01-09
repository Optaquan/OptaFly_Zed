use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::path::PathBuf;

/// Configure Zed settings to use Widget-Log proxy
pub async fn configure_zed_settings(auth_token: &str, proxy_url: &str) -> Result<()> {
    log::info!("Configuring Zed settings for Widget-Log");

    let settings_path = get_zed_settings_path();

    // Create directory if needed
    if let Some(parent) = settings_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .context("Failed to create Zed config directory")?;
    }

    // Read existing settings or create new
    let mut settings: Value = if settings_path.exists() {
        let content = tokio::fs::read_to_string(&settings_path)
            .await
            .context("Failed to read Zed settings")?;
        serde_json::from_str(&content).unwrap_or_else(|_| {
            log::warn!("Could not parse existing Zed settings, creating new");
            json!({})
        })
    } else {
        log::info!("Creating new Zed settings file");
        json!({})
    };

    // Merge Widget-Log configuration
    settings["language_models"] = json!({
        "anthropic": {
            "version": "1",
            "api_url": proxy_url,
            "low_speed_timeout_in_seconds": 60,
            "available_models": [
                {
                    "name": "claude-3-5-sonnet-20241022",
                    "display_name": "Claude 3.5 Sonnet (Cached)",
                    "max_tokens": 8192,
                    "max_output_tokens": 8192,
                    "max_cache_anchors": 4,
                    "cache_configuration": {
                        "max_cache_anchors": 4,
                        "min_total_token": 2048,
                        "min_tokens_per_anchor": 1024,
                        "should_speculate": true,
                        "cache_ttl_seconds": 300
                    },
                    "tool_override": {
                        "max_tokens": 8192
                    }
                },
                {
                    "name": "claude-opus-4-20250514",
                    "display_name": "Claude Opus 4 (Cached)",
                    "max_tokens": 8192,
                    "max_output_tokens": 8192,
                    "max_cache_anchors": 4,
                    "cache_configuration": {
                        "max_cache_anchors": 4,
                        "min_total_token": 2048,
                        "min_tokens_per_anchor": 1024,
                        "should_speculate": true,
                        "cache_ttl_seconds": 300
                    }
                }
            ]
        }
    });

    settings["http_client"] = json!({
        "http2": true,
        "timeout_in_seconds": 120,
        "headers": {
            "Authorization": format!("Bearer {}", auth_token)
        }
    });

    // Write back
    let content = serde_json::to_string_pretty(&settings)
        .context("Failed to serialize settings")?;
    tokio::fs::write(&settings_path, content)
        .await
        .context("Failed to write Zed settings")?;

    log::info!("Zed settings configured successfully at: {}", settings_path.display());
    Ok(())
}

/// Get authentication token from Widget-Log .env file
pub async fn get_auth_token(widget_log_dir: &PathBuf) -> Result<String> {
    let env_file = widget_log_dir.join(".env");
    if !env_file.exists() {
        anyhow::bail!(".env file not found at: {}", env_file.display());
    }

    let content = tokio::fs::read_to_string(&env_file)
        .await
        .context("Failed to read .env file")?;

    for line in content.lines() {
        if line.starts_with("WIDGET_LOG_AUTH_TOKEN=") {
            let token = line.trim_start_matches("WIDGET_LOG_AUTH_TOKEN=").trim();
            if !token.is_empty() {
                return Ok(token.to_string());
            }
        }
    }

    anyhow::bail!("WIDGET_LOG_AUTH_TOKEN not found in .env file")
}

fn get_zed_settings_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zed")
        .join("settings.json")
}
