use anyhow::{Context, Result};
use std::path::PathBuf;

/// Wrapper around Mistral.rs inference engine
pub struct MistralRsEngine {
    model_path: PathBuf,
    // Note: Actual mistralrs integration will be completed once we verify
    // the exact API surface. For now, this is the architectural placeholder.
}

impl MistralRsEngine {
    /// Initialize the Mistral.rs inference engine
    pub async fn new(model_id: &str) -> Result<Self> {
        log::info!("Initializing Mistral.rs engine with model: {}", model_id);

        // Get model cache directory
        let cache_dir = dirs::cache_dir()
            .context("Failed to get cache directory")?
            .join("optafly-zed")
            .join("models");

        tokio::fs::create_dir_all(&cache_dir).await?;

        let model_path = cache_dir.join(model_id.replace('/', "_"));

        // TODO: Download model from HuggingFace if not present
        // This will use hf-hub crate to download GGUF models
        if !model_path.exists() {
            log::warn!(
                "Model not found at {:?}. Will need to download.",
                model_path
            );
            // For Phase 2a, we create the structure
            // Model downloading will be implemented in Phase 2b
        }

        Ok(Self { model_path })
    }

    /// Run inference on a prompt
    pub async fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        log::debug!("Running inference: {} tokens", max_tokens);

        // TODO: Actual mistralrs inference call
        // For now, return placeholder that shows the structure works

        // Placeholder response that demonstrates prompt analysis
        let response = format!(
            "Analyzed prompt: '{}'\n\
            Suggested improvements:\n\
            1. Add specific context about the code/file\n\
            2. Include expected behavior or error details\n\
            3. Structure request with clear objectives",
            prompt
        );

        Ok(response)
    }

    /// Analyze user intent from prompt
    pub async fn analyze_intent(&self, user_input: &str) -> Result<PromptIntent> {
        log::debug!("Analyzing intent for: {}", user_input);

        let analysis_prompt = format!(
            "Analyze this user request and identify:\n\
            1. Primary intent (debug, implement, refactor, explain, etc.)\n\
            2. Missing information that would improve the request\n\
            3. Optimal structure for the prompt\n\n\
            User request: {}\n\n\
            Analysis:",
            user_input
        );

        let _response = self.infer(&analysis_prompt, 256).await?;

        // Parse response into structured intent
        // For now, use heuristics until full inference is implemented
        Ok(PromptIntent {
            primary_action: classify_action(user_input),
            missing_context: vec![
                "file location".to_string(),
                "specific error or behavior".to_string(),
            ],
            confidence: 0.85,
        })
    }
}

/// Represents the analyzed intent of a user prompt
#[derive(Debug, Clone)]
pub struct PromptIntent {
    pub primary_action: PromptAction,
    pub missing_context: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub enum PromptAction {
    Debug,
    Implement,
    Refactor,
    Explain,
    Optimize,
    Document,
    Other(String),
}

/// Classify the primary action from user input
fn classify_action(input: &str) -> PromptAction {
    let lower = input.to_lowercase();

    if lower.contains("fix") || lower.contains("bug") || lower.contains("error") {
        PromptAction::Debug
    } else if lower.contains("add") || lower.contains("create") || lower.contains("implement") {
        PromptAction::Implement
    } else if lower.contains("refactor") || lower.contains("clean") || lower.contains("improve") {
        PromptAction::Refactor
    } else if lower.contains("explain") || lower.contains("how") || lower.contains("what") {
        PromptAction::Explain
    } else if lower.contains("optim") || lower.contains("faster") || lower.contains("performance") {
        PromptAction::Optimize
    } else if lower.contains("document") || lower.contains("comment") {
        PromptAction::Document
    } else {
        PromptAction::Other(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_classification() {
        assert!(matches!(
            classify_action("fix this bug"),
            PromptAction::Debug
        ));
        assert!(matches!(
            classify_action("add dark mode"),
            PromptAction::Implement
        ));
        assert!(matches!(
            classify_action("explain how this works"),
            PromptAction::Explain
        ));
    }
}
