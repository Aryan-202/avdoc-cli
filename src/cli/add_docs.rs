use anyhow::Result;
use colored::*;
use std::path::PathBuf;

use crate::ai::llm_client::LLMClient;
use crate::models::repo::Repository;
use crate::scanner::repo_scanner::RepoScanner;

pub async fn run(path: PathBuf) -> Result<()> {
    println!("{}", "📚 Adding AI-powered documentation to your project...".bright_blue().bold());

    // Scan repository
    let scanner = RepoScanner::new(path.clone())?;
    let files = scanner.scan()?;
    
    // Build repository model
    let repo = Repository::build(path.clone(), files).await?;
    
    // Initialize LLM client
    let client = LLMClient::from_env()?;

    println!("{}", format!("Analyzing {} files...", repo.files.len()).dimmed());

    for file in repo.files {
        let file_path = file.path.to_string_lossy().to_string();
        
        // Skip certain extensions
        if !["rs", "js", "ts", "py", "cpp", "c", "java", "go", "php", "kt"].contains(&file.language.to_lowercase().as_str()) {
            continue;
        }

        println!("{} {}...", "📝 Documenting".cyan(), file_path.bold());

        let content = std::fs::read_to_string(&file.path)?;
        
        // Generate documentation
        match client.generate_documentation(&content, &file.language).await {
            Ok(documented_content) => {
                std::fs::write(&file.path, documented_content)?;
                println!("  {}", "✅ Done!".green());
            }
            Err(e) => {
                println!("  {} {}", "❌ Error:".red(), e);
            }
        }
    }

    println!("\n{}", "✨ All available documentation has been added!".bright_green().bold());
    Ok(())
}
