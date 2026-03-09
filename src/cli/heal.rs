use anyhow::Result;
use colored::*;
use dialoguer::Confirm;
use std::path::PathBuf;

use crate::ai::llm_client::LLMClient;
use crate::analyzer::doc_score::DocScoreAnalyzer;
use crate::models::repo::Repository;
use crate::scanner::repo_scanner::RepoScanner;

pub async fn run(path: PathBuf, files: Option<Vec<String>>, interactive: bool) -> Result<()> {
    println!("{}", "🩺 Starting auto-heal process...".bright_blue());

    // Scan repository
    let scanner = RepoScanner::new(path.clone())?;
    let all_files = scanner.scan()?;

    // Filter files if specified
    let files_to_heal: Vec<_> = if let Some(specific_files) = files {
        all_files
            .into_iter()
            .filter(|f| specific_files.contains(&f.path.to_string_lossy().to_string()))
            .collect()
    } else {
        // Find low-scoring files
        let repo = Repository::build(path.clone(), all_files).await?;
        let analyzer = DocScoreAnalyzer::new();
        let score_result = analyzer.analyze(&repo).await?;

        repo.files
            .into_iter()
            .filter(|f| {
                score_result
                    .file_scores
                    .get(&f.path.to_string_lossy().to_string())
                    .unwrap_or(&100)
                    < &60
            })
            .collect()
    };

    if files_to_heal.is_empty() {
        println!("{}", "✨ No files need healing!".green());
        return Ok(());
    }

    println!(
        "{}",
        format!("Found {} files that need attention", files_to_heal.len()).yellow()
    );

    // Initialize LLM client
    let client = LLMClient::from_env()?;

    // Process each file
    for file in files_to_heal {
        let file_path = file.path.to_string_lossy().to_string();

        // Check if we should proceed
        if interactive {
            let proceed = Confirm::new()
                .with_prompt(format!("Heal {}?", file_path))
                .interact()?;

            if !proceed {
                continue;
            }
        }

        println!("{}", format!("Healing {}...", file_path).cyan());

        // Generate documentation using AI
        let content = std::fs::read_to_string(&file.path)?;
        let documented_content = client
            .generate_documentation(&content, &file.language)
            .await?;

        // Write back to file
        std::fs::write(&file.path, documented_content)?;

        println!("{}", format!("✅ Healed {}", file_path).green());
    }

    println!("\n{}", "🎉 Healing complete!".bright_green().bold());
    Ok(())
}
