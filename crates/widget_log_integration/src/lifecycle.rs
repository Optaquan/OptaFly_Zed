use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use tokio::time::{sleep, Duration};

pub struct WidgetLogProcess {
    process: Option<Child>,
    port: u16,
    host: String,
}

impl WidgetLogProcess {
    pub fn new(port: u16, host: String) -> Self {
        Self {
            process: None,
            port,
            host,
        }
    }

    /// Start Widget-Log proxy server
    pub async fn start(&mut self, widget_log_dir: PathBuf) -> Result<()> {
        log::info!("Starting Widget-Log proxy...");

        // Check if Python is available
        let python = self.detect_python()?;
        log::info!("Using Python: {}", python);

        // Prepare environment
        let env_file = widget_log_dir.join(".env");
        if !env_file.exists() {
            log::info!("Creating default .env file");
            self.create_default_env(&env_file).await?;
        }

        // Check if proxy script exists
        let proxy_script = widget_log_dir.join("app/secure_proxy.py");
        if !proxy_script.exists() {
            anyhow::bail!(
                "Widget-Log proxy script not found at: {}",
                proxy_script.display()
            );
        }

        // Start proxy
        log::info!("Launching proxy at {}:{}", self.host, self.port);
        let child = Command::new(&python)
            .arg(&proxy_script)
            .arg("--host")
            .arg(&self.host)
            .arg("--port")
            .arg(self.port.to_string())
            .current_dir(&widget_log_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start Widget-Log proxy")?;

        self.process = Some(child);

        // Wait for proxy to be ready
        self.wait_for_ready().await?;

        log::info!("Widget-Log proxy started successfully");
        Ok(())
    }

    /// Stop Widget-Log proxy server
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            log::info!("Stopping Widget-Log proxy");
            process.kill().context("Failed to stop Widget-Log proxy")?;
        }
        Ok(())
    }

    /// Check if proxy is healthy
    pub async fn health_check(&self) -> Result<bool> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(2))
            .build()?;

        let url = format!("https://{}:{}/health", self.host, self.port);
        match client.get(&url).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn detect_python(&self) -> Result<String> {
        // Try python3, then python
        for cmd in &["python3", "python"] {
            if Command::new(cmd)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return Ok(cmd.to_string());
            }
        }
        anyhow::bail!("Python 3 not found. Please install Python 3.8+")
    }

    async fn create_default_env(&self, env_file: &PathBuf) -> Result<()> {
        let auth_token = self.generate_auth_token();
        let env_content = format!(
            "ANTHROPIC_API_KEY=\nWIDGET_LOG_AUTH_TOKEN={}\nWIDGET_LOG_CACHE_DIR=\n",
            auth_token
        );
        tokio::fs::write(env_file, env_content)
            .await
            .context("Failed to create .env file")?;
        log::info!("Generated authentication token");
        Ok(())
    }

    fn generate_auth_token(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        base64::encode(&bytes)
    }

    async fn wait_for_ready(&self) -> Result<()> {
        log::info!("Waiting for Widget-Log proxy to be ready...");

        for i in 0..30 {
            sleep(Duration::from_secs(1)).await;
            if self.health_check().await? {
                log::info!("Widget-Log proxy ready after {} seconds", i + 1);
                return Ok(());
            }
        }
        anyhow::bail!("Widget-Log proxy failed to start within 30 seconds")
    }

    pub fn get_proxy_url(&self) -> String {
        format!("https://{}:{}/v1", self.host, self.port)
    }
}

impl Drop for WidgetLogProcess {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
