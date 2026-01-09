use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: u64,
    pub session_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
}

impl TelemetryEvent {
    pub fn new(session_id: String, event_type: String, payload: serde_json::Value) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            timestamp,
            session_id,
            event_type,
            payload,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConvergedPayload {
    pub iterations: usize,
    pub final_temperature: f32,
    pub duration_ms: u64,
    pub node_count: usize,
    pub edge_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDetectedPayload {
    pub pattern_type: String,
    pub severity: f32,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutQualityPayload {
    pub graph_hash: String,
    pub edge_crossings: usize,
    pub overlap_penalty: f32,
    pub quality_score: f32,
}

pub struct TelemetryLogger {
    session_id: String,
    file: Arc<Mutex<Option<File>>>,
    enabled: bool,
}

impl TelemetryLogger {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            file: Arc::new(Mutex::new(None)),
            enabled: false,
        }
    }

    pub fn with_file(mut self, path: PathBuf) -> crate::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        *self.file.lock().unwrap() = Some(file);
        self.enabled = true;
        Ok(self)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn log_event(&self, event_type: String, payload: serde_json::Value) -> crate::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let event = TelemetryEvent::new(self.session_id.clone(), event_type, payload);
        let json = serde_json::to_string(&event)?;

        if let Some(ref mut file) = *self.file.lock().unwrap() {
            writeln!(file, "{}", json)?;
            file.flush()?;
        }

        Ok(())
    }

    pub fn log_layout_converged(
        &self,
        iterations: usize,
        final_temperature: f32,
        duration_ms: u64,
        node_count: usize,
        edge_count: usize,
    ) -> crate::Result<()> {
        let payload = LayoutConvergedPayload {
            iterations,
            final_temperature,
            duration_ms,
            node_count,
            edge_count,
        };
        self.log_event(
            "layout.converged".to_string(),
            serde_json::to_value(payload)?,
        )
    }

    pub fn log_pattern_detected(
        &self,
        pattern_type: String,
        severity: f32,
        node_ids: Vec<String>,
    ) -> crate::Result<()> {
        let payload = PatternDetectedPayload {
            pattern_type,
            severity,
            node_ids,
        };
        self.log_event(
            "pattern.detected".to_string(),
            serde_json::to_value(payload)?,
        )
    }

    pub fn log_layout_quality(
        &self,
        graph_hash: String,
        edge_crossings: usize,
        overlap_penalty: f32,
        quality_score: f32,
    ) -> crate::Result<()> {
        let payload = LayoutQualityPayload {
            graph_hash,
            edge_crossings,
            overlap_penalty,
            quality_score,
        };
        self.log_event("layout.quality".to_string(), serde_json::to_value(payload)?)
    }
}

impl Default for TelemetryLogger {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_telemetry_disabled_by_default() {
        let logger = TelemetryLogger::default();
        assert!(!logger.is_enabled());
    }

    #[test]
    fn test_telemetry_enable_disable() {
        let mut logger = TelemetryLogger::default();
        logger.enable();
        assert!(logger.is_enabled());
        logger.disable();
        assert!(!logger.is_enabled());
    }

    #[test]
    fn test_log_event_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let logger = TelemetryLogger::new("test-session".to_string())
            .with_file(path.clone())
            .unwrap();

        logger
            .log_event(
                "test.event".to_string(),
                serde_json::json!({"key": "value"}),
            )
            .unwrap();

        let mut content = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("test.event"));
        assert!(content.contains("test-session"));
        assert!(content.contains("\"key\":\"value\""));
    }

    #[test]
    fn test_log_layout_converged() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let logger = TelemetryLogger::new("test-session".to_string())
            .with_file(path.clone())
            .unwrap();

        logger.log_layout_converged(50, 0.01, 123, 10, 15).unwrap();

        let mut content = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("layout.converged"));
        assert!(content.contains("\"iterations\":50"));
        assert!(content.contains("\"node_count\":10"));
    }

    #[test]
    fn test_log_pattern_detected() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let logger = TelemetryLogger::new("test-session".to_string())
            .with_file(path.clone())
            .unwrap();

        logger
            .log_pattern_detected(
                "Cycle".to_string(),
                0.8,
                vec!["A".to_string(), "B".to_string()],
            )
            .unwrap();

        let mut content = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("pattern.detected"));
        assert!(content.contains("\"pattern_type\":\"Cycle\""));
        assert!(content.contains("\"severity\":0.8"));
    }
}
