use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Context extracted from Zed workspace
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZedContext {
    pub active_file: Option<ActiveFile>,
    pub selected_text: Option<Selection>,
    pub project_info: Option<ProjectInfo>,
    pub recent_edits: Vec<RecentEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveFile {
    pub path: PathBuf,
    pub language: Option<String>,
    pub cursor_line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root_path: PathBuf,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentEdit {
    pub file: PathBuf,
    pub description: String,
    pub timestamp: i64,
}

impl ZedContext {
    /// Create context from current Zed state
    pub fn from_zed_workspace(
        active_file: Option<ActiveFile>,
        selected_text: Option<Selection>,
        project_info: Option<ProjectInfo>,
    ) -> Self {
        Self {
            active_file,
            selected_text,
            project_info,
            recent_edits: Vec::new(),
        }
    }

    /// Get token count estimate for this context
    pub fn estimated_tokens(&self) -> usize {
        let mut tokens = 0;

        if let Some(file) = &self.active_file {
            tokens += file.path.to_string_lossy().len() / 4 + 5;
        }

        if let Some(selection) = &self.selected_text {
            tokens += selection.content.len() / 4;
        }

        if let Some(project) = &self.project_info {
            tokens += project.name.len() / 4 + 5;
        }

        tokens
    }

    /// Check if context is useful for refinement
    pub fn is_useful(&self) -> bool {
        self.active_file.is_some() || self.selected_text.is_some()
    }
}
