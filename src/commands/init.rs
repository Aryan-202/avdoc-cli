use anyhow::{Context, Ok, Result};
use owo_colors::OwoColorize;
use std::{fs, path::{Path, PathBuf}};
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AvdocConfig {
    pub project_name: String,
    pub version: String,
    pub context_dirs: Vec<String>,
    pub ignore_patterns: Vec<String>,
}

impl Default for AvdocConfig {
    fn default() -> Self {
        Self {
            project_name: "my-project".to_string(),
            version: "0.1.0".to_string(),
            context_dirs: vec!["src".to_string(), ".".to_string()],
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
        }
    }
}

pub fn init(path: Option<PathBuf>) -> Result<()> {
    let target_dir = path.unwrap_or_else(|| PathBuf::from("."));
    let target_dir = target_dir.canonicalize()?;

    let avdoc_dir = target_dir.join(".avdoc");
    let config_path = avdoc_dir.join("config.toml");
    let context_path = avdoc_dir.join("context.json");

    if avdoc_dir.exists() {
        println!(
            "{}",
            "avdoc already initialized in this directory!".yellow()
        );
        return Ok(());
    }

    println!("{}", "Initializing avdoc...".cyan().bold());

    fs::create_dir_all(&avdoc_dir)
        .with_context(|| format!("failed to create {}", avdoc_dir.display()))?;

    let project_name = target_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-project")
        .to_string();

    let mut config = AvdocConfig::default();
    config.project_name = project_name;

    if target_dir.join("Cargo.toml").exists() {
        config.context_dirs = vec!["src".to_string(), "Cargo.toml".to_string()];
        config.ignore_patterns.push("target".to_string());
        println!("{}", "Detected Rust project".green());
    }

    if target_dir.join("package.json").exists() {
        config.context_dirs.push("package.json".to_string());
        config.ignore_patterns.push("node_modules".to_string());
        println!("{}", "Detected Node.js project".green());
    }

    if target_dir.join("requirements.txt").exists() || target_dir.join("pyproject.toml").exists() {
        config.context_dirs.push("requirements.txt".to_string());
        config.ignore_patterns.push("__pycache__".to_string());
        config.ignore_patterns.push("*.pyc".to_string());
        println!("{}", "Detected Python project".green());
    }

    let config_content = toml::to_string_pretty(&config)
        .context("Failed to serialize config")?;

    let mut config_file = fs::File::create(&config_path)
        .with_context(|| format!("Failed to create {}", config_path.display()))?;

    config_file.write_all(config_content.as_bytes())?;
    println!("{}", format!("Created {}", config_path.display()).green());


    let context_file = fs::File::create(&context_path)
        .with_context(|| format!("Failed to create {}", context_path.display()))?;

    serde_json::to_writer_pretty(context_file, &config.context_dirs)?;
    println!("{}", format!("Created {}", context_path.display()).green());

    let gitignore_path = target_dir.join(".gitignore");
    if gitignore_path.exists() {
        let mut gitignore = fs::OpenOptions::new()
            .append(true)
            .open(&gitignore_path)?;
        
        writeln!(gitignore, "\n# avdoc\n.avdoc/")?;
        println!("{}", "Added .avdoc/ to .gitignore".green());
    } else {
        let mut gitignore = fs::File::create(&gitignore_path)?;
        writeln!(gitignore, "# avdoc\n.avdoc/")?;
        println!("{}", "Created .gitignore with avdoc exclusion".green());
    }

    println!("\n{}", "avdoc initialized successfully!".green().bold());

    Ok(())
}


pub fn get_config(project_path: &Path) -> Result<AvdocConfig> {
    let config_path = project_path.join(".avdoc/config.toml");
    
    if !config_path.exists() {
        anyhow::bail!("avdoc not initialized. Run 'avdoc init' first");
    }
    
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    
    let config: AvdocConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", config_path.display()))?;
    
    Ok(config)
}

pub fn get_context(project_path: &Path) -> Result<Vec<String>> {
    let context_path = project_path.join(".avdoc/context.json");
    
    if !context_path.exists() {
        return Ok(vec!["src".to_string()]);
    }
    
    let content = fs::read_to_string(&context_path)?;
    let context: Vec<String> = serde_json::from_str(&content)?;
    
    Ok(context)
}

pub fn save_context(project_path: &Path, context: &[String]) -> Result<()> {
    let context_path = project_path.join(".avdoc/context.json");
    let file = fs::File::create(&context_path)?;
    serde_json::to_writer_pretty(file, context)?;
    Ok(())
}
