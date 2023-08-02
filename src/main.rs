use std::{fs};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::{Path, PathBuf};

fn main() {
    let path = "./";
    let pattern = "test";
    rgrep(path, pattern);
}

fn rgrep(path: &str, keyword: &str) {
    let mut searched = Vec::new();
    let files = get_all_files_in_directory(path);
    for file in files {
        match search_file(file.to_str().unwrap(), keyword) {
            Ok(data) => {searched.push(data);}
            Err(_) => {}
        }
    }

    println!("{:?}", searched);
}

fn get_all_files_in_directory(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = Path::new(path);

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.is_file() {
                    files.push(file_path.to_owned());
                } else if file_path.is_dir() {
                    let mut subdirectory_files = get_all_files_in_directory(file_path.to_str().unwrap());
                    files.append(&mut subdirectory_files);
                }
            }
        }
    }

    files
}

fn search_file(path: &str, pattern: &str) -> Result<Vec<(String, usize)>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut searched_files: Vec<(String, usize)> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            searched_files.push((path.to_string(), line_number + 1));
        }
    }

    Ok(searched_files)
}