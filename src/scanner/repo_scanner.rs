use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

use crate::models::repo::SourceFile;

pub struct RepoScanner {
    root: PathBuf,
    ignore_patterns: Vec<String>,
}

impl RepoScanner {
    pub fn new(root: PathBuf) -> Result<Self> {
        Ok(Self {
            root,
            ignore_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".git".to_string(),
                "venv".to_string(),
                "__pycache__".to_string(),
            ],
        })
    }
    
    pub fn scan(&self) -> Result<Vec<SourceFile>> {
        let mut files = Vec::new();
        
        for entry in WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .build()
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && self.should_include(path) {
                if let Some(language) = self.detect_language(path) {
                    files.push(SourceFile {
                        path: path.to_path_buf(),
                        language,
                        content: std::fs::read_to_string(path).ok(),
                    });
                }
            }
        }
        
        Ok(files)
    }
    
    fn should_include(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check ignore patterns
        for pattern in &self.ignore_patterns {
            if path_str.contains(pattern) {
                return false;
            }
        }
        
        // Check file extensions
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "rs" | "go" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "h" | "hpp"
            )
        } else {
            false
        }
    }
    
    fn detect_language(&self, path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "rs" => "rust".to_string(),
                "go" => "go".to_string(),
                "py" => "python".to_string(),
                "js" => "javascript".to_string(),
                "ts" => "typescript".to_string(),
                "java" => "java".to_string(),
                "cpp" | "cc" => "cpp".to_string(),
                "c" => "c".to_string(),
                "h" | "hpp" => "header".to_string(),
                _ => "unknown".to_string(),
            })
    }
}