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