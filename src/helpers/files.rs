use std::{fs, io, path::Path};
use toml::value::Datetime;

pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Datetime,
}

pub fn get_files(path: &Path) -> io::Result<Vec<FileInfo>> {
    let mut data = Vec::new();
    let read_dir = fs::read_dir(path)?;

    for entry in read_dir {
        if let Ok(entry) = entry {
            let name = entry.file_name().into_string().unwrap_or_else(|_| "unknown".into());
            
            let (is_dir, size, modified) = if let Ok(meta) = entry.metadata() {
                
                let toml_datetime = Datetime {
                    date: None, 
                    time: None,
                    offset: None,
                };
                (meta.is_dir(), meta.len(), toml_datetime)
            } else {
                (false, 0, Datetime { date: None, time: None, offset: None })
            };

            data.push(FileInfo {
                name,
                is_dir,
                size,
                modified,
            });
        }
    }


    data.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    Ok(data)
}


pub fn format_size(size: u64) -> String {
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