use links_generator::text_parse::text_parse_fn;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_file_from_path<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?; // Open the file
    let mut content = String::new(); // String to hold the file content
    file.read_to_string(&mut content)?; // Read the file's content
    Ok(content) // Return the content
}

fn main() {
    let path = "src/SUMMARY.md";
    match read_file_from_path(path) {
        Ok(content) => {
            // println!("File content:\n{}", content);
            text_parse_fn(&content);
        }
        Err(e) => println!("Failed to read the file: {}", e),
    }
}
