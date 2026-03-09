use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;

use crate::scanner::repo_scanner::RepoScanner;
use crate::analyzer::doc_score::{DocScoreAnalyzer, ScoreResult};
use crate::models::repo::Repository;

pub async fn run(path: PathBuf, min_score: Option<u8>, format: String) -> Result<()> {
    println!("{}", "🔍 Scanning repository...".bright_blue());
    let start = Instant::now();
    
    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Analyzing files...");
    
    // Scan repository
    let scanner = RepoScanner::new(path.clone())?;
    let files = scanner.scan()?;
    pb.finish_with_message(format!("Found {} files", files.len()));
    
    // Build repository model
    let repo = Repository::build(path, files).await?;
    
    // Analyze documentation score
    let analyzer = DocScoreAnalyzer::new();
    let score_result = analyzer.analyze(&repo).await?;
    
    let duration = start.elapsed();
    
    // Output based on format
    match format.as_str() {
        "json" => print_json(&score_result)?,
        "markdown" => print_markdown(&score_result)?,
        _ => print_terminal(&score_result, duration, min_score)?,
    }
    
    // Check against minimum score if specified
    if let Some(min) = min_score {
        if score_result.overall_score < min {
            std::process::exit(1);
        }
    }
    
    Ok(())
}

fn print_terminal(result: &ScoreResult, duration: std::time::Duration, min_score: Option<u8>) -> Result<()> {
    println!("\n{}", "📊 Documentation Score Report".bright_green().bold());
    println!("{}", "═".repeat(50).bright_black());
    
    // Overall score with color coding
    let score = result.overall_score;
    let score_color = if score >= 80 {
        "green"
    } else if score >= 60 {
        "yellow"
    } else {
        "red"
    };
    
    println!(
        "{}: {}",
        "Overall Score".bright_white().bold(),
        format!("{}/100", score).color(score_color).bold()
    );
    
    if let Some(min) = min_score {
        let status = if score >= min {
            "PASSED".green()
        } else {
            "FAILED".red()
        };
        println!("{}: {} (minimum: {})", "Status".bright_white(), status, min);
    }
    
    println!("\n{}", "File Breakdown".bright_cyan());
    for (file, file_score) in &result.file_scores {
        let file_score_color = if *file_score >= 80 {
            "green"
        } else if *file_score >= 60 {
            "yellow"
        } else {
            "red"
        };
        println!("  {}: {}", file, format!("{}/100", file_score).color(file_score_color));
    }
    
    println!("\n{}", "Metrics".bright_cyan());
    println!("  {}: {}", "Total Files".bright_white(), result.total_files);
    println!("  {}: {}", "Documented Functions".bright_white(), result.documented_functions);
    println!("  {}: {}", "Undocumented Functions".bright_white(), result.undocumented_functions);
    println!("  {}: {:.2}s", "Analysis Time".bright_white(), duration.as_secs_f64());
    
    println!("\n{}", "Recommendations".bright_yellow());
    for rec in &result.recommendations {
        println!("  • {}", rec);
    }
    
    Ok(())
}

fn print_json(result: &ScoreResult) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(result)?);
    Ok(())
}

fn print_markdown(result: &ScoreResult) -> Result<()> {
    println!("# Documentation Score Report\n");
    println!("## Overall Score: **{}/100**", result.overall_score);
    println!("\n## File Breakdown\n");
    println!("| File | Score |");
    println!("|------|-------|");
    for (file, score) in &result.file_scores {
        println!("| {} | {}/100 |", file, score);
    }
    println!("\n## Recommendations\n");
    for rec in &result.recommendations {
        println!("- {}", rec);
    }
    Ok(())
}