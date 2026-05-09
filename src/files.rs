use std::{fs};
use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::PathBuf;
use crate::parser::LogEntry;

pub fn read_file_contents(path: &PathBuf) -> io::Result<Vec<LogEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parsed = LogEntry::parse(&line);
        result.push(parsed);
    }
    Ok(result)

}
