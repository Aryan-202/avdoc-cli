use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;

use crate::ai::llm_client::LLMClient;
use crate::models::repo::Repository;
use crate::scanner::repo_scanner::RepoScanner;

pub async fn run(path: PathBuf) -> Result<()> {
    println!("{}", "🏗️  Analyzing project for README generation...".bright_blue());
    
    // Scan repository to understand context
    let scanner = RepoScanner::new(path.clone())?;
    let files = scanner.scan()?;
    
    // Build repository model
    let repo = Repository::build(path.clone(), files).await?;
    
    // Initialize LLM client
    let client = LLMClient::from_env()?;
    
    println!("{}", "✨ Generating project README with AI...".cyan());
    
    // Collect some context about the project (e.g. file names, languages, etc.)
    let context_summary: String = repo.files.iter()
        .take(20) // Limit context
        .map(|f| format!("{}: {}", f.path.to_string_lossy(), f.language))
        .collect::<Vec<String>>()
        .join("\n");
        
    let prompt = format!(
        "Generate a comprehensive and professional README.md for a project with the following files and structure:\n\n{}\n\nInclude sections for: Project Name, Description, Tech Stack, Features, and Getting Started. Return only the markdown content.",
        context_summary
    );
    
    // Use raw chat or similar to get the README
    let readme_content = client.chat_generic(&prompt).await?;
    
    let readme_path = path.join("README.md");
    fs::write(&readme_path, readme_content)?;
    
    println!("\n{}", "✅ README.md generated/updated successfully!".bright_green().bold());
    Ok(())
}