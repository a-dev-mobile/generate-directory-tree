mod models {
    pub mod argument_parser;
    pub mod directory_scanner;
}

mod utilities {
    pub mod file_content_printer;
}


use models::argument_parser::ArgumentParser;
use models::directory_scanner::DirectoryScanner;
use utilities::file_content_printer::FileContentPrinter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arguments = ArgumentParser::parse(&args[1..]);

    if let Err(validation_message) = arguments.is_valid() {
        println!("Error: {}", validation_message);
        return;
    }

    let mut scanner = DirectoryScanner::new(&arguments.scan_dir(), &arguments.exclude_items());
    scanner.scan_and_print_directory_tree();

    let mut printer = FileContentPrinter::new(scanner.file_paths_to_display, &arguments.scan_dir());
    if arguments.show_contents() {
        // Assuming FileContentPrinter has been modified to include the process_files method
        printer.process_files(); // This is the updated line
    }

    printer.print_character_counts();
}