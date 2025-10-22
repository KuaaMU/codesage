//! Code metrics analyzer

use codesage_core::{
    AnalysisContext, Analyzer, CodeMetrics, Issue, IssueCategory, Location, Result,
    Severity,
};
use std::collections::HashSet;

/// Analyzer for code metrics and complexity
pub struct MetricsAnalyzer;

impl MetricsAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate cyclomatic complexity (simplified version)
    fn calculate_cyclomatic_complexity(source: &str) -> u32 {
        let mut complexity = 1u32;

        // Count decision points
        for line in source.lines() {
            let line = line.trim();
            // Keywords that increase complexity
            if line.contains("if ") || line.contains("else if") {
                complexity += 1;
            }
            if line.contains("while ") || line.contains("for ") {
                complexity += 1;
            }
            if line.contains("match ") {
                complexity += 1;
            }
            if line.contains("&&") || line.contains("||") {
                complexity += line.matches("&&").count() as u32;
                complexity += line.matches("||").count() as u32;
            }
            if line.contains('?') {
                complexity += 1;
            }
        }

        complexity
    }

    /// Calculate cognitive complexity (simplified)
    fn calculate_cognitive_complexity(source: &str) -> u32 {
        let mut complexity = 0u32;
        let mut nesting_level = 0u32;

        for line in source.lines() {
            let line = line.trim();

            // Track nesting
            if line.contains('{') {
                nesting_level += 1;
            }
            if line.contains('}') {
                nesting_level = nesting_level.saturating_sub(1);
            }

            // Add complexity based on control structures and nesting
            if line.contains("if ") || line.contains("else if") {
                complexity += nesting_level + 1;
            }
            if line.contains("while ") || line.contains("for ") {
                complexity += nesting_level + 1;
            }
        }

        complexity
    }

    /// Calculate maintainability index (simplified)
    fn calculate_maintainability_index(source: &str, cyclomatic: u32) -> f32 {
        let lines = source.lines().count() as f32;
        let volume = lines * (cyclomatic as f32).ln();

        // Simplified MI = 171 - 5.2 * ln(V) - 0.23 * G - 16.2 * ln(LOC)
        let mi = 171.0 - 5.2 * volume.ln() - 0.23 * (cyclomatic as f32) - 16.2 * lines.ln();
        mi.clamp(0.0, 100.0)
    }

    /// Detect code duplication (simplified)
    fn calculate_duplication_percentage(source: &str) -> f32 {
        let lines: Vec<&str> = source
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
            .collect();

        if lines.is_empty() {
            return 0.0;
        }

        let mut seen = HashSet::new();
        let mut duplicates = 0;

        for line in &lines {
            if !seen.insert(line.trim()) {
                duplicates += 1;
            }
        }

        (duplicates as f32 / lines.len() as f32) * 100.0
    }

    /// Calculate technical debt in minutes
    fn calculate_technical_debt(
        cyclomatic: u32,
        cognitive: u32,
        duplication: f32,
        maintainability: f32,
    ) -> u32 {
        let mut debt_minutes = 0u32;

        // High complexity adds debt
        if cyclomatic > 10 {
            debt_minutes += (cyclomatic - 10) * 5;
        }
        if cognitive > 15 {
            debt_minutes += (cognitive - 15) * 3;
        }

        // High duplication adds debt
        if duplication > 5.0 {
            debt_minutes += (duplication * 2.0) as u32;
        }

        // Low maintainability adds debt
        if maintainability < 65.0 {
            debt_minutes += ((65.0 - maintainability) * 2.0) as u32;
        }

        debt_minutes
    }

    /// Generate metrics for the source code
    pub fn calculate_metrics(&self, source: &str) -> CodeMetrics {
        let lines_of_code = source.lines().count();
        let cyclomatic = Self::calculate_cyclomatic_complexity(source);
        let cognitive = Self::calculate_cognitive_complexity(source);
        let maintainability = Self::calculate_maintainability_index(source, cyclomatic);
        let duplication = Self::calculate_duplication_percentage(source);
        let technical_debt =
            Self::calculate_technical_debt(cyclomatic, cognitive, duplication, maintainability);

        CodeMetrics {
            lines_of_code,
            cyclomatic_complexity: cyclomatic,
            cognitive_complexity: cognitive,
            maintainability_index: maintainability,
            test_coverage: None,
            duplication_percentage: duplication,
            technical_debt_minutes: technical_debt,
        }
    }
}

impl Default for MetricsAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer for MetricsAnalyzer {
    fn name(&self) -> &str {
        "metrics"
    }

    fn analyze(&self, context: &AnalysisContext) -> Result<Vec<Issue>> {
        let metrics = self.calculate_metrics(&context.source_code);
        let mut issues = Vec::new();

        // Generate issues based on metrics
        if metrics.cyclomatic_complexity > 10 {
            issues.push(Issue {
                id: "COMPLEXITY001".to_string(),
                severity: if metrics.cyclomatic_complexity > 20 {
                    Severity::P1
                } else {
                    Severity::P2
                },
                category: IssueCategory::Maintainability,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: metrics.lines_of_code,
                    end_column: 1,
                },
                message: format!(
                    "High cyclomatic complexity: {}",
                    metrics.cyclomatic_complexity
                ),
                explanation: "This code has high cyclomatic complexity, making it harder to understand and test. Consider breaking it into smaller functions.".to_string(),
                fix_suggestion: None,
                confidence: 0.9,
            });
        }

        if metrics.cognitive_complexity > 15 {
            issues.push(Issue {
                id: "COMPLEXITY002".to_string(),
                severity: Severity::P2,
                category: IssueCategory::Maintainability,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: metrics.lines_of_code,
                    end_column: 1,
                },
                message: format!(
                    "High cognitive complexity: {}",
                    metrics.cognitive_complexity
                ),
                explanation: "This code has high cognitive complexity with deep nesting. Consider refactoring to reduce nesting levels.".to_string(),
                fix_suggestion: None,
                confidence: 0.85,
            });
        }

        if metrics.maintainability_index < 65.0 {
            issues.push(Issue {
                id: "MAINTAINABILITY001".to_string(),
                severity: Severity::P2,
                category: IssueCategory::Maintainability,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: metrics.lines_of_code,
                    end_column: 1,
                },
                message: format!(
                    "Low maintainability index: {:.1}",
                    metrics.maintainability_index
                ),
                explanation: "This code has a low maintainability index. Consider refactoring to improve code quality.".to_string(),
                fix_suggestion: None,
                confidence: 0.8,
            });
        }

        if metrics.duplication_percentage > 10.0 {
            issues.push(Issue {
                id: "DUPLICATION001".to_string(),
                severity: Severity::P3,
                category: IssueCategory::Maintainability,
                location: Location {
                    file_path: context.file_path.clone(),
                    start_line: 1,
                    start_column: 1,
                    end_line: metrics.lines_of_code,
                    end_column: 1,
                },
                message: format!(
                    "Code duplication detected: {:.1}%",
                    metrics.duplication_percentage
                ),
                explanation: "Duplicate code has been detected. Consider extracting common code into reusable functions.".to_string(),
                fix_suggestion: None,
                confidence: 0.7,
            });
        }

        Ok(issues)
    }
}
