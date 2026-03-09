use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct ReadmeUpdater {
    path: PathBuf,
    content: String,
}

impl ReadmeUpdater {
    pub fn new(path: PathBuf) -> Self {
        let content = if path.exists() {
            fs::read_to_string(&path).unwrap_or_default()
        } else {
            String::new()
        };
        
        Self { path, content }
    }
    
    pub fn update_with_diagram(&mut self, diagram: &str, title: &str) -> Result<()> {
        let diagram_section = format!("\n## {}\n\n```mermaid\n{}\n```\n", title, diagram);
        
        // Check if diagram section already exists
        if self.content.contains(&format!("## {}", title)) {
            // Replace existing diagram section
            let re = regex::Regex::new(&format!(r"(?s)## {0}.*?(?=## |$)", title)).unwrap();
            self.content = re.replace(&self.content, &diagram_section).to_string();
        } else {
            // Add diagram section at the end or after overview
            if let Some(pos) = self.content.find("## Overview") {
                if let Some(end_pos) = self.content[pos..].find("\n## ") {
                    let insert_pos = pos + end_pos;
                    self.content.insert_str(insert_pos, &diagram_section);
                } else {
                    self.content.push_str(&diagram_section);
                }
            } else {
                self.content.push_str(&diagram_section);
            }
        }
        
        fs::write(&self.path, &self.content)?;
        Ok(())
    }
    
    pub fn update_with_doc_score(&mut self, score: u8) -> Result<()> {
        let badge = format!("![Doc Score](https://img.shields.io/badge/doc%20score-{}-{})", 
            score,
            if score >= 80 { "brightgreen" } else if score >= 60 { "yellow" } else { "red" }
        );
        
        // Add badge at the top if it doesn't exist
        if !self.content.contains("![Doc Score]") {
            self.content = format!("{}\n\n{}", badge, self.content);
        }
        
        fs::write(&self.path, &self.content)?;
        Ok(())
    }
}