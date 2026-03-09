use anyhow::Result;
use colored::*;
use std::path::PathBuf;

use crate::scanner::repo_scanner::RepoScanner;
use crate::analyzer::architecture::ArchitectureAnalyzer;
use crate::diagram::mermaid::MermaidGenerator;
use crate::markdown::readme_updater::ReadmeUpdater;
use crate::models::repo::Repository;
use crate::ai::llm_client::LLMClient;

pub async fn run(path: PathBuf, format: String, update_readme: bool) -> Result<()> {
    println!("{}", "🏗️  Analyzing architecture...".bright_blue());
    
    // Scan repository
    let scanner = RepoScanner::new(path.clone())?;
    let files = scanner.scan()?;
    
    // Build repository model
    let repo = Repository::build(path.clone(), files).await?;
    
    // Analyze architecture
    let analyzer = ArchitectureAnalyzer::new();
    let architecture = analyzer.analyze(&repo).await?;
    
    // Generate diagram
    let diagram = match format.as_str() {
        "ascii" => generate_ascii_diagram(&architecture)?,
        _ => generate_mermaid_diagram(&repo, &architecture).await?,
    };
    
    println!("\n{}", diagram);
    
    // Update README if requested
    if update_readme {
        let readme_path = path.join("README.md");
        let mut updater = ReadmeUpdater::new(readme_path);
        updater.update_with_diagram(&diagram, "Architecture Diagram")?;
        println!("\n{}", "✅ README.md updated with diagram".green());
    }
    
    Ok(())
}

fn generate_ascii_diagram(architecture: &crate::models::service::Architecture) -> Result<String> {
    let mut diagram = String::new();
    diagram.push_str("┌─ Architecture Diagram ──────────────────────┐\n");
    
    // Services
    for service in &architecture.services {
        diagram.push_str(&format!("│ {:<40} │\n", format!("📦 {}", service.name)));
        
        // Dependencies
        for dep in &service.dependencies {
            diagram.push_str(&format!("│   └─▶ {:<36} │\n", dep));
        }
        
        if !service.dependencies.is_empty() {
            diagram.push_str("│                                            │\n");
        }
    }
    
    diagram.push_str("└────────────────────────────────────────────┘\n");
    Ok(diagram)
}

async fn generate_mermaid_diagram(
    repo: &Repository,
    architecture: &crate::models::service::Architecture,
) -> Result<String> {
    let client = LLMClient::from_env()?;
    let generator = MermaidGenerator::new(client);
    generator.generate(repo, architecture).await
}