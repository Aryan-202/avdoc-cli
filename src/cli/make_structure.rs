use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub async fn run(path: PathBuf) -> Result<()> {
    println!("{}", "🏗️ Generating project structure...".bright_blue().bold());
    
    let mut structure = String::new();
    structure.push_str("# Project Structure\n\n");
    structure.push_str("```text\n");
    
    let root_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("root");
    
    structure.push_str(&format!("{}/\n", root_name));
    
    let entries: Vec<_> = WalkDir::new(&path)
        .min_depth(1)
        .max_depth(3) // reasonable depth
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map_or(false, |s| s.starts_with('.') || s == "target" || s == "node_modules"))
        .filter_map(|e| e.ok())
        .collect();
        
    for entry in entries {
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy();
        let indent = "  ".repeat(depth);
        let is_dir = entry.file_type().is_dir();
        
        if is_dir {
            structure.push_str(&format!("{}{} /\n", indent, name));
        } else {
            structure.push_str(&format!("{}{} \n", indent, name));
        }
    }
    
    structure.push_str("```\n");
    
    let output_path = path.join("structure.md");
    fs::write(&output_path, structure)?;
    
    println!("{} {}", "✅ Folder structure generated in:".green(), output_path.to_string_lossy().bold().cyan());
    Ok(())
}
