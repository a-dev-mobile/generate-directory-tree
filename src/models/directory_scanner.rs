use std::fs::{self, DirEntry};
use std::path::PathBuf;

pub struct DirectoryScanner {
    scan_dir: PathBuf,
    exclude_items: Vec<String>,
    pub file_paths_to_display: Vec<PathBuf>,
}

impl DirectoryScanner {
    pub fn new(scan_dir: &str, exclude_items: &[String]) -> Self {
        Self {
            scan_dir: PathBuf::from(scan_dir),
            exclude_items: exclude_items.to_vec(),
            file_paths_to_display: Vec::new(),
        }
    }

    pub fn scan_and_print_directory_tree(&mut self) {
        if !self.scan_dir.exists() {
            println!(
                "Error: The specified scan directory '{:?}' does not exist.",
                self.scan_dir
            );
            return;
        }

        println!("{}\n", self.scan_dir.display());
        println!("{}\n", "Directory Tree");
        self.print_directory_tree(self.scan_dir.clone(), String::new(), true);
    }

      fn print_directory_tree(&mut self, dir_path: PathBuf, indent: String, is_root: bool) {
        let entries = match fs::read_dir(&dir_path) {
            Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<DirEntry>>(),
            Err(_) => return,
        };

        if entries.is_empty() && !is_root {
            return;
        }

        for (i, entry) in entries.iter().enumerate() {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if self.exclude_items.contains(&file_name) {
                continue;
            }

            let is_directory = entry.path().is_dir();
            
            if is_directory {
                println!("{}{}\\", indent, file_name);
    
                // Check if this is the last directory/file in the current directory
                let new_indent = if i == entries.len() - 1 {
                    format!("{}    ", indent.replace("|", " ")) // Replace '|' with ' ' for the last item
                } else {
                    format!("{}|   ", indent) // Add '|' for non-last items
                };
                self.print_directory_tree(entry.path(), new_indent, false);
            } else {
                println!("{}- {}", indent, file_name);
                self.file_paths_to_display.push(entry.path());
            }
        }
    }
    
    
}
