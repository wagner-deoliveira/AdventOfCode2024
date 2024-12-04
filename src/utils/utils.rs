use std::fs::File;
use std::io::{Error, Read, BufReader, BufRead};
use std::path::Path;

pub fn read_file(file_path: &str) -> Result<String, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).map_err(|e| {
        println!("Error opening file: {}", e);
        e
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_lines(file_path: &str) -> Result<Vec<String>, Error> {
    let path = Path::new(file_path);
    let file = File::open(&path).map_err(|e| {
        println!("Error opening file: {}", e);
        e
    })?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn read_as_bytes(file_path: &str) -> Result<Vec<u8>, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).map_err(|e| {
        println!("Error opening file: {}", e);
        e
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}
