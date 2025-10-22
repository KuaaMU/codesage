//! CodeSage CLI Library
//!
//! This module provides the CLI entry point and command handlers

use clap::{Parser, Subcommand};
use codesage_analyzer::{AnalysisEngine, MetricsAnalyzer};
use codesage_core::{AnalysisContext, Result};
use codesage_parser::CodeParser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codesage")]
#[command(about = "AI-powered code review and refactoring tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Review code for issues and improvements
    Review {
        /// Path to file or directory to review
        path: String,

        /// Review recursively
        #[arg(short, long)]
        recursive: bool,

        /// Output format (text, json, sarif)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Enable AI-powered review (requires ANTHROPIC_API_KEY)
        #[arg(long)]
        ai: bool,
    },

    /// Perform intelligent refactoring
    Refactor {
        /// Path to file to refactor
        path: String,

        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Generate technical debt report
    Debt {
        /// Path to analyze
        path: String,

        /// Output HTML report
        #[arg(long)]
        output_html: Option<String>,
    },

    /// Fix issues automatically
    Fix {
        /// Path to fix
        path: String,

        /// Only fix specific categories
        #[arg(long)]
        category: Option<String>,

        /// Auto-apply safe fixes
        #[arg(long)]
        auto_apply: bool,
    },
}

/// Main CLI entry point
pub async fn run() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Review {
            path,
            recursive,
            format,
            ai,
        } => {
            handle_review(path, recursive, format, ai).await?;
        }
        Commands::Refactor { path, interactive } => {
            println!("{} Refactoring: {}", "♻️".green(), path.bold());
            println!("   Interactive: {}", interactive);
            println!("\n{} Refactoring feature coming soon!", "⚠️".yellow());
        }
        Commands::Debt { path, output_html } => {
            println!("{} Analyzing technical debt: {}", "📊".blue(), path.bold());
            if let Some(output) = output_html {
                println!("   Output: {}", output);
            }
            println!("\n{} Debt analysis feature coming soon!", "⚠️".yellow());
        }
        Commands::Fix {
            path,
            category,
            auto_apply,
        } => {
            println!("{} Fixing issues in: {}", "🔧".magenta(), path.bold());
            if let Some(cat) = category {
                println!("   Category: {}", cat);
            }
            println!("   Auto-apply: {}", auto_apply);
            println!("\n{} Auto-fix feature coming soon!", "⚠️".yellow());
        }
    }

    Ok(())
}

/// Handle the review command
async fn handle_review(path: String, _recursive: bool, format: String, use_ai: bool) -> Result<()> {
    println!("{} Reviewing code at: {}", "🔍".cyan(), path.bold());

    let file_path = PathBuf::from(&path);

    // Parse the file
    let parser = CodeParser::new();
    let parsed = parser.parse_file(&file_path)?;

    println!("\n{}", "Analysis Results:".bold().underline());
    println!("  Language: {:?}", parsed.language);
    println!("  Lines of code: {}", parsed.line_count());

    // Create analysis context
    let context = AnalysisContext {
        file_path: file_path.clone(),
        source_code: parsed.source().to_string(),
        language: parsed.language,
    };

    // Run static analysis
    let mut engine = AnalysisEngine::new();
    engine.register_analyzer(Box::new(MetricsAnalyzer::new()));

    let issues = engine.analyze(&context)?;

    // Display results based on format
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&issues)
                .map_err(|e| codesage_core::CodeSageError::Unknown(e.to_string()))?;
            println!("\n{}", json);
        }
        "text" => {
            if issues.is_empty() {
                println!("\n{} No issues found!", "✓".green().bold());
            } else {
                println!("\n{} Found {} issue(s):", "⚠".yellow().bold(), issues.len());
                for (i, issue) in issues.iter().enumerate() {
                    println!(
                        "\n{}. [{}] {}",
                        i + 1,
                        format!("{:?}", issue.severity).bold(),
                        issue.message
                    );
                    println!("   Category: {:?}", issue.category);
                    println!(
                        "   Location: {}:{}",
                        issue.location.file_path.display(),
                        issue.location.start_line
                    );
                    println!("   {}", issue.explanation);
                }
            }
        }
        _ => {
            eprintln!("Unknown format: {}. Using text format.", format);
            if issues.is_empty() {
                println!("\n{} No issues found!", "✓".green().bold());
            } else {
                println!("\n{} Found {} issue(s):", "⚠".yellow().bold(), issues.len());
                for (i, issue) in issues.iter().enumerate() {
                    println!(
                        "\n{}. [{}] {}",
                        i + 1,
                        format!("{:?}", issue.severity).bold(),
                        issue.message
                    );
                    println!("   Category: {:?}", issue.category);
                    println!(
                        "   Location: {}:{}",
                        issue.location.file_path.display(),
                        issue.location.start_line
                    );
                    println!("   {}", issue.explanation);
                }
            }
        }
    }

    // AI review if enabled
    if use_ai {
        println!("\n{} Running AI-powered review...", "🤖".cyan());

        use codesage_ai::{AIClient, AIConfig};
        use codesage_core::AIReviewer;

        let ai_client = AIClient::with_config(AIConfig::default());
        match ai_client.review(&context).await {
            Ok(review_result) => {
                println!("\n{} AI Review Complete", "✓".green().bold());
                if !review_result.issues.is_empty() {
                    println!(
                        "\nAI found {} additional insight(s):",
                        review_result.issues.len()
                    );
                    for (i, issue) in review_result.issues.iter().enumerate() {
                        println!("\n{}. {}", i + 1, issue.message.bold());
                        println!("   {}", issue.explanation);
                    }
                }
            }
            Err(e) => {
                eprintln!("\n{} AI review unavailable: {}", "⚠".yellow(), e);
                eprintln!(
                    "   Tip: Set ANTHROPIC_API_KEY environment variable to enable AI features"
                );
            }
        }
    }

    Ok(())
}
