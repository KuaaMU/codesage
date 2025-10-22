//! Core data models for CodeSage

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a code issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub severity: Severity,
    pub category: IssueCategory,
    pub location: Location,
    pub message: String,
    pub explanation: String,
    pub fix_suggestion: Option<Fix>,
    pub confidence: f32,
}

/// Severity level of an issue
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    P0, // Critical
    P1, // High
    P2, // Medium
    P3, // Low
}

/// Category of code issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Bug,
    Security,
    Performance,
    Maintainability,
    Style,
    Documentation,
    TestCoverage,
}

/// Location in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file_path: PathBuf,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Fix suggestion for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fix {
    pub description: String,
    pub diff: String,
    pub safe_to_auto_apply: bool,
}

/// Code metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub maintainability_index: f32,
    pub test_coverage: Option<f32>,
    pub duplication_percentage: f32,
    pub technical_debt_minutes: u32,
}

/// Result of code review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewResult {
    pub file_path: PathBuf,
    pub issues: Vec<Issue>,
    pub metrics: CodeMetrics,
    pub suggestions: Vec<Suggestion>,
    pub timestamp: String,
}

/// Refactoring suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub title: String,
    pub description: String,
    pub refactoring_type: RefactoringType,
    pub before_code: String,
    pub after_code: String,
    pub impact: Impact,
}

/// Type of refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringType {
    ExtractMethod,
    ExtractVariable,
    InlineMethod,
    RenameSymbol,
    SimplifyConditional,
    RemoveDeadCode,
    IntroduceDesignPattern,
}

/// Impact of a change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Impact {
    High,
    Medium,
    Low,
}

/// Supported programming languages
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    CPP,
    CSharp,
}

impl Language {
    /// Get file extensions for this language
    pub fn extensions(&self) -> &[&str] {
        match self {
            Language::Rust => &["rs"],
            Language::JavaScript => &["js", "jsx"],
            Language::TypeScript => &["ts", "tsx"],
            Language::Python => &["py"],
            Language::Go => &["go"],
            Language::Java => &["java"],
            Language::CPP => &["cpp", "cc", "cxx", "hpp", "h"],
            Language::CSharp => &["cs"],
        }
    }

    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Language::Rust),
            "js" | "jsx" => Some(Language::JavaScript),
            "ts" | "tsx" => Some(Language::TypeScript),
            "py" => Some(Language::Python),
            "go" => Some(Language::Go),
            "java" => Some(Language::Java),
            "cpp" | "cc" | "cxx" | "hpp" | "h" => Some(Language::CPP),
            "cs" => Some(Language::CSharp),
            _ => None,
        }
    }
}
