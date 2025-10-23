//! CodeSage CLI Library
//!
//! This module provides the CLI entry point and command handlers

use clap::{Parser, Subcommand};
use codesage_analyzer::{AnalysisEngine, MetricsAnalyzer};
use codesage_core::{AnalysisContext, Issue, Result};
use codesage_parser::CodeParser;
use colored::Colorize;
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
async fn handle_review(path: String, recursive: bool, format: String, use_ai: bool) -> Result<()> {
    let path_buf = PathBuf::from(&path);

    if recursive && path_buf.is_dir() {
        handle_recursive_review(path_buf, format, use_ai).await
    } else {
        handle_single_file_review(path_buf, format, use_ai).await
    }
}

/// Handle review of a single file
async fn handle_single_file_review(
    file_path: PathBuf,
    format: String,
    use_ai: bool,
) -> Result<()> {
    println!("{} Reviewing code at: {}", "🔍".cyan(), file_path.display().to_string().bold());

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
    display_results(&issues, &format);

    // AI review if enabled
    if use_ai {
        run_ai_review(&context).await;
    }

    Ok(())
}

/// Handle recursive review of a directory
async fn handle_recursive_review(
    dir_path: PathBuf,
    format: String,
    use_ai: bool,
) -> Result<()> {
    println!(
        "{} Recursively reviewing directory: {}",
        "🔍".cyan(),
        dir_path.display().to_string().bold()
    );

    // Collect all source files
    let files = collect_source_files(&dir_path)?;

    if files.is_empty() {
        println!("\n{} No source files found!", "⚠".yellow().bold());
        return Ok(());
    }

    println!(
        "\n{} Found {} file(s) to review",
        "📁".cyan(),
        files.len()
    );

    // Setup progress bar
    let progress = ProgressBar::new(files.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("=>-"),
    );

    // Analyze files in parallel
    let all_issues = Arc::new(Mutex::new(Vec::new()));
    let parser = CodeParser::new();

    files.par_iter().for_each(|file_path| {
        progress.set_message(format!("Analyzing {}", file_path.file_name().unwrap_or_default().to_string_lossy()));

        if let Ok(parsed) = parser.parse_file(file_path) {
            let context = AnalysisContext {
                file_path: file_path.clone(),
                source_code: parsed.source().to_string(),
                language: parsed.language,
            };

            let mut engine = AnalysisEngine::new();
            engine.register_analyzer(Box::new(MetricsAnalyzer::new()));

            if let Ok(issues) = engine.analyze(&context)
                && !issues.is_empty() {
                    let mut all = all_issues.lock().unwrap();
                    all.extend(issues);
                }
        }

        progress.inc(1);
    });

    progress.finish_with_message("Analysis complete");

    let issues = all_issues.lock().unwrap();

    // Display aggregated results
    println!("\n{}", "Summary:".bold().underline());
    println!("  Files analyzed: {}", files.len());
    println!("  Total issues found: {}", issues.len());

    if !issues.is_empty() {
        // Group issues by severity
        let mut p0_count = 0;
        let mut p1_count = 0;
        let mut p2_count = 0;
        let mut p3_count = 0;

        for issue in issues.iter() {
            match issue.severity {
                codesage_core::Severity::P0 => p0_count += 1,
                codesage_core::Severity::P1 => p1_count += 1,
                codesage_core::Severity::P2 => p2_count += 1,
                codesage_core::Severity::P3 => p3_count += 1,
            }
        }

        println!("\n{}", "Issues by severity:".bold());
        if p0_count > 0 {
            println!("  P0 (Critical): {}", p0_count.to_string().red().bold());
        }
        if p1_count > 0 {
            println!("  P1 (High): {}", p1_count.to_string().yellow().bold());
        }
        if p2_count > 0 {
            println!("  P2 (Medium): {}", p2_count);
        }
        if p3_count > 0 {
            println!("  P3 (Low): {}", p3_count);
        }

        display_results(&issues, &format);
    } else {
        println!("\n{} No issues found!", "✓".green().bold());
    }

    // AI review for recursive mode
    if use_ai {
        println!("\n{} AI review is not yet supported for recursive mode", "⚠".yellow());
        println!("   Tip: Use --ai with single file review for AI-powered insights");
    }

    Ok(())
}

/// Collect all source files from a directory, respecting .gitignore
fn collect_source_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // Supported extensions
    let supported_extensions = vec![
        "rs", "js", "ts", "jsx", "tsx", "py", "go", "java", "cpp", "cc", "cxx", "c", "cs",
    ];

    for result in WalkBuilder::new(dir)
        .hidden(false) // Include hidden files
        .git_ignore(true) // Respect .gitignore
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                        && supported_extensions.contains(&ext.to_string_lossy().as_ref()) {
                            files.push(path.to_path_buf());
                        }
            }
            Err(err) => {
                eprintln!("{} Error walking directory: {}", "⚠".yellow(), err);
            }
        }
    }

    Ok(files)
}

/// Display analysis results in the requested format
fn display_results(issues: &[Issue], format: &str) {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&issues)
                .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e));
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
}

/// Run AI-powered review
async fn run_ai_review(context: &AnalysisContext) {
    println!("\n{} Running AI-powered review...", "🤖".cyan());

    use codesage_ai::{AIClient, AIConfig};
    use codesage_core::AIReviewer;

    let ai_client = AIClient::with_config(AIConfig::default());
    match ai_client.review(context).await {
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
