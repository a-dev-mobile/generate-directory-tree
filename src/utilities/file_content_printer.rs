use std::{fs, path::PathBuf, collections::HashMap};

pub struct FileContentPrinter {
    file_paths: Vec<PathBuf>,
    scan_dir: PathBuf,
    file_character_counts: HashMap<String, usize>,
}

impl FileContentPrinter {
    pub fn new(file_paths: Vec<PathBuf>, scan_dir: &str) -> Self {
        Self {
            file_paths,
            scan_dir: PathBuf::from(scan_dir),
            file_character_counts: HashMap::new(),
        }
    }

   pub fn process_files(&mut self) {
    // First, print the contents of each file
    for file_path in &self.file_paths {
        self.print_file_contents_with_header(file_path);
    }

   


}

    fn print_file_contents_with_header(&self, file_path: &PathBuf) {
        let relative_path = file_path
            .strip_prefix(&self.scan_dir)
            .unwrap_or_else(|_| file_path)
            .display();
        println!("\nFile: {}\n", relative_path);
        match fs::read_to_string(file_path) {
            Ok(contents) => println!("    {}", contents.replace('\n', "\n    ")),
            Err(e) => println!("    Failed to read file: {}", e),
        }
    }

    fn count_file_characters(&mut self, file_path: &PathBuf) {
        let relative_path = file_path
            .strip_prefix(&self.scan_dir)
            .unwrap_or_else(|_| file_path)
            .to_string_lossy()
            .into_owned();
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                self.file_character_counts.insert(relative_path, contents.len());
            }
            Err(e) => println!("    Failed to read file for character counting: {}", e),
        }
    }

    pub fn print_character_counts(mut self) {
         // Clone file paths for character counting to avoid borrow checker issues
    let file_paths_clone = self.file_paths.clone();

    // Now, count characters in each file
    for file_path in file_paths_clone {
        self.count_file_characters(&file_path);
    }
        println!("\nCharacter Counts (Descending)\n");
        let mut counts: Vec<(&String, &usize)> = self.file_character_counts.iter().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));

        for (file, &count) in counts {
            println!("{}: {}", count, file);
        }
    }
}
