mod cli;
use avdoc::helpers;

use clap::Parser;
use cli::{Cli, Commands};
use owo_colors::OwoColorize;

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls { path } => {
            match helpers::files::get_files(&path) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("{}", "Directory is empty.".yellow());
                        return;
                    }
                    let max_name_len = files.iter().map(|f| f.name.len()).max().unwrap_or(20).max(10);
                    let name_col_width = std::cmp::min(max_name_len, 50) + 2;

                    let type_col_width = 8;
                    let size_col_width = 12;

                    let header_name = format!("{:<width$}", "Name", width = name_col_width);
                    let header_type = format!("{:<width$}", "Type", width = type_col_width);
                    let header_size = format!("{:<width$}", "Size", width = size_col_width);
                    
                    println!(
                        "{} | {} | {}",
                        header_name.bold().underline(),
                        header_type.bold().underline(),
                        header_size.bold().underline()
                    );
                    
                    let separator_len = name_col_width + type_col_width + size_col_width + 6;
                    println!("{}", "-".repeat(separator_len));

                    for file in files {
                        let display_name = if file.name.len() > name_col_width - 2 {
                            let mut truncated = file.name[..name_col_width - 5].to_string();
                            truncated.push_str("...");
                            truncated
                        } else {
                            file.name.clone()
                        };

                        let padded_name = format!("{:<width$}", display_name, width = name_col_width);
                        let padded_type = format!("{:<width$}", if file.is_dir { "Dir" } else { "File" }, width = type_col_width);
                        let padded_size = format!("{:<width$}", if file.is_dir { "-".to_string() } else { format_size(file.size) }, width = size_col_width);

                        let final_name = if file.is_dir {
                            padded_name.blue().bold().to_string()
                        } else {
                            padded_name
                        };

                        let final_type = if file.is_dir {
                            padded_type.magenta().to_string()
                        } else {
                            padded_type.green().to_string()
                        };

                        let final_size = padded_size.cyan().to_string();

                        println!("{} | {} | {}", final_name, final_type, final_size);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Failed to list directory:".red().bold(), e);
                }
                
            }
        },
        Commands::Init { path } => {
            if let Err(e) = avdoc::commands::init::init(path) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
    
}