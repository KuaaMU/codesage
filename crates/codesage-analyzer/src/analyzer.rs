//! Analysis engine implementation

use codesage_core::{AnalysisContext, Analyzer, Issue, Result};

/// Main analysis engine
pub struct AnalysisEngine {
    analyzers: Vec<Box<dyn Analyzer>>,
}

impl AnalysisEngine {
    /// Create a new analysis engine
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// Register an analyzer
    pub fn register_analyzer(&mut self, analyzer: Box<dyn Analyzer>) {
        self.analyzers.push(analyzer);
    }

    /// Run all analyzers on the given context
    pub fn analyze(&self, context: &AnalysisContext) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();

        for analyzer in &self.analyzers {
            let issues = analyzer.analyze(context)?;
            all_issues.extend(issues);
        }

        Ok(all_issues)
    }
}

impl Default for AnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}
