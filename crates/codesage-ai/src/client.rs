//! AI client implementation

use async_trait::async_trait;
use codesage_core::{
    AIReviewer, AnalysisContext, CodeMetrics, CodeReviewResult, CodeSageError, Issue,
    IssueCategory, Location, Result, Severity,
};
use serde::{Deserialize, Serialize};

/// Configuration for AI client
#[derive(Debug, Clone)]
pub struct AIConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub api_base_url: String,
    pub timeout_seconds: u64,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_base_url: "https://api.anthropic.com/v1".to_string(),
            timeout_seconds: 60,
        }
    }
}

/// AI client for code review
pub struct AIClient {
    config: AIConfig,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

impl AIClient {
    /// Create a new AI client with default configuration
    pub fn new() -> Self {
        Self::with_config(AIConfig::default())
    }

    /// Create a new AI client with custom configuration
    pub fn with_config(config: AIConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Build the review prompt
    fn build_review_prompt(&self, context: &AnalysisContext) -> String {
        let lang = format!("{:?}", context.language);
        let lang_lower = lang.to_lowercase();
        format!(
            r#"Please review the following {lang} code and provide a structured analysis:

File: {}
Lines of code: {}

Code:
```{lang_lower}
{}
```

Please analyze for:
1. Potential bugs and logic errors
2. Security vulnerabilities
3. Performance issues
4. Code quality and maintainability
5. Best practices violations

Provide specific, actionable feedback."#,
            context.file_path.display(),
            context.source_code.lines().count(),
            context.source_code
        )
    }

    /// Call Claude API (mock implementation for now)
    async fn call_claude_api(&self, prompt: String) -> Result<String> {
        // Check if API key is available
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| CodeSageError::AIError("ANTHROPIC_API_KEY not set".to_string()))?;

        let request = ClaudeRequest {
            model: self.config.model.clone(),
            max_tokens: 4096,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post(format!("{}/messages", self.config.api_base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| CodeSageError::AIError(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CodeSageError::AIError(format!(
                "API error {}: {}",
                status, error_text
            )));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| CodeSageError::AIError(format!("Failed to parse response: {}", e)))?;

        Ok(claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }

    /// Parse AI response into issues (simplified)
    fn parse_ai_response(&self, response: &str, context: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Simple keyword-based parsing
        // In a real implementation, this would use structured output from the AI

        if response.to_lowercase().contains("security")
            || response.to_lowercase().contains("vulnerability")
        {
            issues.push(Issue {
                id: "AI_SECURITY001".to_string(),
                severity: Severity::P1,
                category: IssueCategory::Security,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: context.source_code.lines().count(),
                    end_column: 1,
                },
                message: "Potential security concern identified by AI".to_string(),
                explanation: response.to_string(),
                fix_suggestion: None,
                confidence: 0.75,
            });
        }

        if response.to_lowercase().contains("bug") || response.to_lowercase().contains("error") {
            issues.push(Issue {
                id: "AI_BUG001".to_string(),
                severity: Severity::P2,
                category: IssueCategory::Bug,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: context.source_code.lines().count(),
                    end_column: 1,
                },
                message: "Potential bug identified by AI".to_string(),
                explanation: response.to_string(),
                fix_suggestion: None,
                confidence: 0.7,
            });
        }

        issues
    }
}

impl Default for AIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIReviewer for AIClient {
    async fn review(&self, context: &AnalysisContext) -> Result<CodeReviewResult> {
        let prompt = self.build_review_prompt(context);

        // Try to call the API, but provide a fallback for when API key is not available
        let ai_response = match self.call_claude_api(prompt).await {
            Ok(response) => response,
            Err(e) => {
                // Fallback to basic analysis when API is not available
                eprintln!("Warning: AI analysis unavailable: {}", e);
                "AI analysis unavailable. Please set ANTHROPIC_API_KEY environment variable."
                    .to_string()
            }
        };

        let issues = self.parse_ai_response(&ai_response, context);

        Ok(CodeReviewResult {
            file_path: context.file_path.clone(),
            issues,
            metrics: CodeMetrics {
                lines_of_code: context.source_code.lines().count(),
                cyclomatic_complexity: 0,
                cognitive_complexity: 0,
                maintainability_index: 0.0,
                test_coverage: None,
                duplication_percentage: 0.0,
                technical_debt_minutes: 0,
            },
            suggestions: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}
