use serde::{Deserialize, Serialize};

/// Result of user approval interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalResult {
    Accepted,
    Modified(String),
    Rejected,
}

impl ApprovalResult {
    /// Get the final prompt to use
    pub fn final_prompt(&self, original: &str, refined: &str) -> String {
        match self {
            ApprovalResult::Accepted => refined.to_string(),
            ApprovalResult::Modified(custom) => custom.clone(),
            ApprovalResult::Rejected => original.to_string(),
        }
    }
}
