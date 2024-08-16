use std::fs;

pub fn write_lines(path: &str, lines: Vec<String>) -> Result<(), String> {
    match fs::write(path, lines.join("\n")) {
        Ok(()) => Ok(()),
        Err(reason) => Err(reason.to_string())
    }
}

pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
    match fs::read_to_string(path) {
        Ok(file) => Ok(file.split("\n").map(|s| { s.to_string() }).collect()),
        Err(reason) => Err(reason.to_string())
    }
}
