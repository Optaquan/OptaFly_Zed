use anyhow::Result;
use pyo3_bridge::types::CacheAnchor;

/// Analyzer for extracting optimal cache anchor positions
pub struct CacheAnchorAnalyzer {}

impl CacheAnchorAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract cache anchors from a prompt
    pub async fn extract_anchors(&self, prompt: &str) -> Result<Vec<CacheAnchor>> {
        log::debug!("Extracting cache anchors from prompt");

        let mut anchors = Vec::new();

        // Strategy: Identify major sections of the prompt as anchors
        // This optimizes for Anthropic's prompt caching feature

        // 1. System-level context (if present)
        if let Some(system_section) = Self::extract_section(prompt, "system", "user") {
            anchors.push(CacheAnchor {
                position: 0,
                content: system_section.clone(),
                priority: 1.0,
                estimated_tokens: system_section.len() / 4,
            });
        }

        // 2. Code context (if present)
        if prompt.contains("```") {
            if let Some(code_section) = Self::extract_code_blocks(prompt) {
                anchors.push(CacheAnchor {
                    position: anchors.len(),
                    content: code_section.clone(),
                    priority: 0.9,
                    estimated_tokens: code_section.len() / 4,
                });
            }
        }

        // 3. File/project context
        if prompt.contains("file") || prompt.contains("src/") {
            if let Some(context_section) = Self::extract_file_context(prompt) {
                anchors.push(CacheAnchor {
                    position: anchors.len(),
                    content: context_section.clone(),
                    priority: 0.8,
                    estimated_tokens: context_section.len() / 4,
                });
            }
        }

        // Limit to 4 anchors (Anthropic's current limit)
        anchors.truncate(4);

        log::info!("Extracted {} cache anchors", anchors.len());
        Ok(anchors)
    }

    fn extract_section(text: &str, start_marker: &str, end_marker: &str) -> Option<String> {
        if let Some(start) = text.find(start_marker) {
            if let Some(end) = text[start..].find(end_marker) {
                return Some(text[start..start + end].to_string());
            }
        }
        None
    }

    fn extract_code_blocks(text: &str) -> Option<String> {
        let mut code_blocks = Vec::new();
        let mut in_code = false;
        let mut current_block = String::new();

        for line in text.lines() {
            if line.starts_with("```") {
                if in_code {
                    code_blocks.push(current_block.clone());
                    current_block.clear();
                }
                in_code = !in_code;
            } else if in_code {
                current_block.push_str(line);
                current_block.push('\n');
            }
        }

        if code_blocks.is_empty() {
            None
        } else {
            Some(code_blocks.join("\n"))
        }
    }

    fn extract_file_context(text: &str) -> Option<String> {
        // Extract sentences containing file references
        let sentences: Vec<&str> = text.split('.').collect();
        let file_context: Vec<&str> = sentences
            .into_iter()
            .filter(|s| s.contains("file") || s.contains("src/") || s.contains(".rs"))
            .collect();

        if file_context.is_empty() {
            None
        } else {
            Some(file_context.join(". "))
        }
    }
}

impl Default for CacheAnchorAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anchor_extraction() {
        let analyzer = CacheAnchorAnalyzer::new();
        let prompt = "In file src/main.rs:\n\n```rust\nfn main() {}\n```\n\nFix the bug";

        let anchors = analyzer.extract_anchors(prompt).await.unwrap();
        assert!(!anchors.is_empty());
    }
}
