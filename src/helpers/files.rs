//! Filesystem inspection and formatting utilities.
//!
//! Provides data structures and helper functions to enumerate directory entries,
//! extract file metadata, sort entries, and format raw byte counts for display.

use std::{fs, io, path::Path};
use toml::value::Datetime;

/// Metadata summary of an individual file or directory entry.
pub struct FileInfo {
    /// Name of the file or directory.
    pub name: String,
    /// Indicates whether the entry is a directory (`true`) or a file (`false`).
    pub is_dir: bool,
    /// Size of the file in bytes (or 0 if metadata is unavailable or directory).
    pub size: u64,
    /// Last modification timestamp representation.
    pub modified: Datetime,
}

/// Reads the entries in a directory and returns a sorted collection of [`FileInfo`].
///
/// Enumerates all immediate children of the specified path. Metadata failures default
/// to non-directory entries with zero size. Results are sorted hierarchically with
/// directories appearing before files, and entries within the same category sorted alphabetically.
///
/// # Returns
///
/// Returns `Ok(Vec<FileInfo>)` with the sorted list of directory entries.
///
/// # Errors
///
/// Returns `Err(std::io::Error)` if the provided path cannot be opened or read as a directory.
pub fn get_files(path: &Path) -> io::Result<Vec<FileInfo>> {
    let mut data = Vec::new();
    let read_dir = fs::read_dir(path)?;

    for entry in read_dir.flatten() {
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


    data.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    Ok(data)
}

/// Converts a byte count into a human-readable formatted string.
///
/// Automatically formats the byte count into binary multiples (GB, MB, KB, or raw bytes).
///
/// # Examples
///
/// ```
/// use avdoc::helpers::files::format_size;
///
/// assert_eq!(format_size(500), "500 B");
/// assert_eq!(format_size(1024), "1.00 KB");
/// assert_eq!(format_size(1048576), "1.00 MB");
/// ```
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