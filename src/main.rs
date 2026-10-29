use std::fs;
use std::io;
use std::path::{Path, PathBuf};
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Enter A Search Keyword");
    } else {
        let current_dir = Path::new(".");
        let mut file_list: Vec<PathBuf> = Vec::new();
        get_all_file(current_dir, &mut file_list)?;
        let word = args[1].to_string().to_lowercase();
        for file in file_list {
            // println!("{}", file.display());
            if let Some(file_lines) = read_file_to_line(&file) {
                search_and_print_file_for_word(&file, file_lines, &word);
            }
        }
    }
    Ok(())
}

fn get_all_file(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                get_all_file(&path, files)?;
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    Ok(())
}

fn read_file_to_line(file_path: &PathBuf) -> Option<Vec<String>> {
    let content = fs::read_to_string(file_path).ok()?;

    let lines: Vec<String> = content.lines().map(String::from).collect();
    Some(lines)
}

fn search_and_print_file_for_word(filepath: &PathBuf, file: Vec<String>, word: &String) {
    let mut said_file_name = false;
    for (index, line) in file.iter().enumerate() {
        if line.to_lowercase().contains(word) {
            if !said_file_name {
                said_file_name = true;
                println!("{}", filepath.display());
            }
            println!("{} : {}", index, line)
        }
    }
}
