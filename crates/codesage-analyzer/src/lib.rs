//! CodeSage Analyzer Library
//!
//! Static and semantic code analysis

pub mod analyzer;
pub mod metrics;

pub use analyzer::AnalysisEngine;
pub use metrics::MetricsAnalyzer;
