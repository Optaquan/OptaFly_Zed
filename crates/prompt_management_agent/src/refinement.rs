use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::context::ZedContext;
use crate::inference::{MistralRsEngine, PromptIntent};
use crate::AgentConfig;

/// Represents a refined prompt with improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinedPrompt {
    pub original: String,
    pub refined: String,
    pub changes: Vec<PromptChange>,
    pub quality_score: f32,
    pub estimated_cache_hit_improvement: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChange {
    pub change_type: ChangeType,
    pub description: String,
    pub tokens_added: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    AddedContext,
    AddedStructure,
    AddedCodeSnippet,
    AddedFileLocation,
    AddedExpectedBehavior,
    ImprovedClarity,
}

/// Helper for generating prompt refinements
pub struct PromptRefinement {
    pub refined_text: String,
    pub changes: Vec<PromptChange>,
    pub quality_score: f32,
}

impl PromptRefinement {
    /// Generate a refined prompt using local inference
    pub async fn generate(
        user_input: &str,
        context: &ZedContext,
        engine: &MistralRsEngine,
        config: &AgentConfig,
    ) -> Result<Self> {
        log::debug!("Generating refinement for: {}", user_input);

        // Analyze intent
        let intent = engine.analyze_intent(user_input).await?;

        // Build refinement based on intent and context
        let mut refined_text = user_input.to_string();
        let mut changes = Vec::new();
        let mut quality_improvement = 0.0f32;

        // Add file context if available and relevant
        if let Some(active_file) = &context.active_file {
            if should_add_file_context(user_input) {
                refined_text = format!(
                    "In file `{}`:\n\n{}",
                    active_file.path.display(),
                    refined_text
                );
                changes.push(PromptChange {
                    change_type: ChangeType::AddedFileLocation,
                    description: format!("Added file context: {}", active_file.path.display()),
                    tokens_added: 8,
                });
                quality_improvement += 0.15;
            }
        }

        // Add code snippet if available
        if let Some(selection) = &context.selected_text {
            if !selection.content.is_empty() && should_add_code_snippet(user_input) {
                refined_text = format!(
                    "{}\n\nRelevant code:\n```{}\n{}\n```",
                    refined_text,
                    context
                        .active_file
                        .as_ref()
                        .and_then(|f| f.language.as_deref())
                        .unwrap_or(""),
                    selection.content
                );
                changes.push(PromptChange {
                    change_type: ChangeType::AddedCodeSnippet,
                    description: "Added selected code snippet for context".to_string(),
                    tokens_added: estimate_tokens(&selection.content) + 10,
                });
                quality_improvement += 0.25;
            }
        }

        // Add structure based on intent
        if intent.confidence > 0.7 {
            let structured = add_structure_for_intent(&refined_text, &intent);
            if structured != refined_text {
                refined_text = structured;
                changes.push(PromptChange {
                    change_type: ChangeType::AddedStructure,
                    description: "Added structured format for clarity".to_string(),
                    tokens_added: 12,
                });
                quality_improvement += 0.20;
            }
        }

        // Calculate final quality score
        let base_quality = calculate_base_quality(user_input);
        let quality_score = (base_quality + quality_improvement).min(1.0);

        Ok(Self {
            refined_text,
            changes,
            quality_score,
        })
    }

    /// Convert to RefinedPrompt with cache hit improvement estimate
    pub fn into_refined_prompt(self, original: &str) -> RefinedPrompt {
        let cache_improvement = estimate_cache_hit_improvement(&self.changes);

        RefinedPrompt {
            original: original.to_string(),
            refined: self.refined_text,
            changes: self.changes,
            quality_score: self.quality_score,
            estimated_cache_hit_improvement: cache_improvement,
        }
    }
}

impl RefinedPrompt {
    /// Get highlighted changes for UI display
    pub fn highlight_changes(&self) -> Vec<ChangeHighlight> {
        self.changes
            .iter()
            .map(|change| ChangeHighlight {
                description: change.description.clone(),
                impact: match change.change_type {
                    ChangeType::AddedCodeSnippet => "High",
                    ChangeType::AddedFileLocation => "Medium",
                    ChangeType::AddedContext => "Medium",
                    ChangeType::AddedStructure => "Medium",
                    ChangeType::AddedExpectedBehavior => "High",
                    ChangeType::ImprovedClarity => "Low",
                },
                tokens: change.tokens_added,
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ChangeHighlight {
    pub description: String,
    pub impact: &'static str,
    pub tokens: usize,
}

fn should_add_file_context(input: &str) -> bool {
    // Don't add if already mentions file
    !input.contains(".rs") && !input.contains("file") && !input.contains("src/")
}

fn should_add_code_snippet(input: &str) -> bool {
    // Add code snippet if asking about specific behavior
    let lower = input.to_lowercase();
    lower.contains("this")
        || lower.contains("here")
        || lower.contains("fix")
        || lower.contains("bug")
}

fn add_structure_for_intent(text: &str, intent: &PromptIntent) -> String {
    use crate::inference::PromptAction;

    match intent.primary_action {
        PromptAction::Debug => {
            format!(
                "{}\n\nPlease:\n1. Identify the root cause\n2. Suggest a fix\n3. Explain how to prevent this in the future",
                text
            )
        }
        PromptAction::Implement => {
            format!(
                "{}\n\nPlease:\n1. Design the approach\n2. Implement the feature\n3. Add appropriate error handling",
                text
            )
        }
        PromptAction::Refactor => {
            format!(
                "{}\n\nPlease:\n1. Analyze current structure\n2. Propose improvements\n3. Implement the refactoring",
                text
            )
        }
        _ => text.to_string(),
    }
}

fn calculate_base_quality(input: &str) -> f32 {
    let mut score: f32 = 0.3; // Base score

    // Longer prompts generally more detailed
    if input.len() > 50 {
        score += 0.1;
    }
    if input.len() > 100 {
        score += 0.1;
    }

    // Contains code snippets
    if input.contains("```") {
        score += 0.2;
    }

    // Contains file references
    if input.contains(".rs") || input.contains("src/") {
        score += 0.15;
    }

    // Contains specific details
    if input.contains("line") || input.contains("function") || input.contains("error") {
        score += 0.15;
    }

    score.min(1.0)
}

fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: 1 token ≈ 4 characters
    text.len() / 4
}

fn estimate_cache_hit_improvement(changes: &[PromptChange]) -> f32 {
    let mut improvement = 0.0f32;

    for change in changes {
        improvement += match change.change_type {
            ChangeType::AddedCodeSnippet => 0.20,
            ChangeType::AddedFileLocation => 0.10,
            ChangeType::AddedContext => 0.15,
            ChangeType::AddedStructure => 0.10,
            ChangeType::AddedExpectedBehavior => 0.15,
            ChangeType::ImprovedClarity => 0.05,
        };
    }

    improvement.min(0.50) // Cap at 50% improvement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_quality_calculation() {
        let short = "fix bug";
        let detailed =
            "Fix the null pointer error in src/main.rs line 45 when processing user input";

        assert!(calculate_base_quality(detailed) > calculate_base_quality(short));
    }

    #[test]
    fn test_token_estimation() {
        let text = "This is a test"; // 14 chars
        assert_eq!(estimate_tokens(text), 3); // ~3 tokens
    }
}
