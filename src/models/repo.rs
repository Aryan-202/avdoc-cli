use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub language: String,
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct Repository {
    pub root: PathBuf,
    pub name: String,
    pub files: Vec<SourceFile>,
}

impl Repository {
    pub async fn build(root: PathBuf, files: Vec<SourceFile>) -> Result<Self> {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Ok(Self {
            root,
            name,
            files,
        })
    }
}