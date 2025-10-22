//! Core traits for CodeSage components

use crate::{CodeReviewResult, Issue, Result};
use std::path::Path;

/// Trait for code analyzers
pub trait Analyzer: Send + Sync {
    /// Name of the analyzer
    fn name(&self) -> &str;

    /// Analyze code and return issues
    fn analyze(&self, context: &AnalysisContext) -> Result<Vec<Issue>>;
}

/// Context for code analysis
pub struct AnalysisContext {
    pub file_path: std::path::PathBuf,
    pub source_code: String,
    pub language: crate::Language,
}

/// Trait for AI-powered code reviewers
#[async_trait::async_trait]
pub trait AIReviewer: Send + Sync {
    /// Perform AI-powered code review
    async fn review(&self, context: &AnalysisContext) -> Result<CodeReviewResult>;
}
