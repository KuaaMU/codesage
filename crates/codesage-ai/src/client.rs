//! AI client implementation

use async_trait::async_trait;
use codesage_core::{AIReviewer, AnalysisContext, CodeReviewResult, Result};

/// AI client for code review
pub struct AIClient {
    // TODO: Add LLM client configuration
}

impl AIClient {
    /// Create a new AI client
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIReviewer for AIClient {
    async fn review(&self, _context: &AnalysisContext) -> Result<CodeReviewResult> {
        // TODO: Implement actual AI review
        todo!("AI review not yet implemented")
    }
}
