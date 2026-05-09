use std::fs;
use std::path::PathBuf;
use crate::parser::LogEntry;

pub fn read_file_contents(path: &PathBuf) -> Vec<LogEntry> {
    let mut result = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        let parsed = LogEntry::parse(line);
        result.push(parsed);
    }
    result

}
