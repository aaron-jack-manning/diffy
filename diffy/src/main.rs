mod diff;

use std::fs;
use std::env;
use std::path;

fn process_arguments() -> Result<(path::PathBuf, path::PathBuf), String> {
    let path1 = match env::args().nth(1) {
        Some(input_path) => path::Path::new(&input_path).to_path_buf(),
        None => return Err(String::from("Input path not specified."))
    };

    let path2 = match env::args().nth(2) {
        Some(input_path) => path::Path::new(&input_path).to_path_buf(),
        None => return Err(String::from("Second input path not specified."))
    };

    Ok((path1, path2))
}

fn comparison(line1 : &&str, line2 : &&str) -> bool {
    line1.trim_start() == line2.trim_start()
}

fn read_file(path : path::PathBuf) -> Result<String, String> {
    match fs::read_to_string(&path) {
        Ok(contents) => Ok(contents),
        Err(_) => Err(format!("Could not read file at {:?}", path)),
    }
}

fn process() -> Result<String, String> {
    let (path1, path2) = process_arguments()?;

    let file1 = read_file(path1)?;
    let file1_lines = file1.split("\n").collect();
    let file2 = read_file(path2)?;
    let file2_lines = file2.split("\n").collect();

    let diff = diff::diff(&file1_lines, &file2_lines, comparison);

    Ok(format!("{}", diff))
}

fn main() {
    match process() {
        Ok(message) => println!("{}", message),
       Err(message) => println!("{}", message),
    }
}
