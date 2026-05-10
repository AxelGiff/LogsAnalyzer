use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use crate::parser::LogEntry;
use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;

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

pub fn print_colored_lines(
    reader: &mut BufReader<File>,
    re_http_method: &Lazy<Regex, fn() -> Regex>,
    re_custom: &Lazy<Regex, fn() -> Regex>,
    re_symfony: &Lazy<Regex, fn() -> Regex>,
    re_syslog: &Lazy<Regex, fn() -> Regex>,

) -> Result<(), Box<dyn std::error::Error>> {
    for ligne in reader.by_ref().lines() {
        let line = ligne?;
        let mut ligne_coloree = line.clone();

        if let Some(captures) = re_http_method.captures(&line) {
            if let Some(date) = captures.name("date") {
                let date = date.as_str();
                let colored = date.green().bold().to_string();
                ligne_coloree = ligne_coloree.replacen(date, &colored, 1);
            }
            if let Some(status) = captures.name("status") {
                let status = status.as_str();
                let colored = match status {
                    "404" | "500" | "503" | "403" => Some(status.red().to_string()),
                    "200" | "203" | "204" => Some(status.green().to_string()),
                    _ => None,
                };

                if let Some(colored) = colored {
                    ligne_coloree = ligne_coloree.replacen(status, &colored, 1);
                }
            }

            if let Some(method) = captures.name("method") {
                let method = method.as_str();
                let colored = match method {
                    "GET" => Some(method.green().to_string()),
                    "POST" | "OPTIONS" => Some(method.blue().to_string()),
                    "PUT" => Some(method.yellow().to_string()),
                    "DELETE" => Some(method.red().to_string()),
                    "PATCH" => Some(method.truecolor(128, 128, 0).to_string()),
                    _ => None,
                };

                if let Some(colored) = colored {
                    ligne_coloree = ligne_coloree.replacen(method, &colored, 1);
                }
            }
        }

        for regex in [re_custom, re_symfony,re_syslog] {
            if let Some(captures) = regex.captures(&line) {
                
                if let Some(host) = captures.name("host") {
                    let host = host.as_str();
                    let colored = host.blue().bold().to_string();
                    ligne_coloree = ligne_coloree.replacen(host, &colored, 1);
                }
                if let Some(process) = captures.name("process") {
                    let process = process.as_str();
                    let colored = process.yellow().to_string();
                    ligne_coloree = ligne_coloree.replacen(process, &colored, 1);
                }
                
                if let Some(date) = captures.name("date") {
                    let date = date.as_str();
                    let colored = date.green().bold().to_string();
                    ligne_coloree = ligne_coloree.replacen(date, &colored, 1);
                }
                
                if let Some(host) = captures.name("host") {
                    let host = host.as_str();
                    let colored = host.blue().bold().to_string();
                    ligne_coloree = ligne_coloree.replacen(host, &colored, 1);
                }
                
                if let Some(process) = captures.name("process") {
                    let process = process.as_str();
                    let colored = process.yellow().to_string();
                    ligne_coloree = ligne_coloree.replacen(process, &colored, 1);
                }
                
                if let Some(level) = captures.name("level") {
                    let level = level.as_str();
                    let colored = match level {
                        "WARNING" => Some(level.yellow().to_string()),
                        "ERROR" | "CRITICAL" => Some(level.red().to_string()),
                        "SUCCESS" => Some(level.green().to_string()),
                        "INFO" => Some(level.blue().to_string()),
                        "DEBUG" => Some(level.cyan().to_string()),
                        _ => None,
                    };

                    if let Some(colored) = colored {
                        ligne_coloree = ligne_coloree.replacen(level, &colored, 1);
                    }
                }
            }
        }

        println!("{}", ligne_coloree);
    }

    Ok(())
}




pub fn print_filtered_colored_lines(
    reader: &String,
    re_http_method: &Lazy<Regex, fn() -> Regex>,
    re_custom: &Lazy<Regex, fn() -> Regex>,
    re_symfony: &Lazy<Regex, fn() -> Regex>,
    re_syslog: &Lazy<Regex, fn() -> Regex>,
) -> Result<(), Box<dyn std::error::Error>> {
    for ligne in reader.lines() {
        let line = ligne.to_string();
        let mut ligne_coloree:String = line.clone();

        if let Some(captures) = re_http_method.captures(&line) {
            if let Some(date) = captures.name("date") {
                let date = date.as_str();
                let colored = date.green().bold().to_string();
                ligne_coloree = ligne_coloree.replacen(date, &colored, 1);
            }

            if let Some(status) = captures.name("status") {
                let status = status.as_str();
                let colored = match status {
                    "404" | "500" | "503" | "403" => Some(status.red().to_string()),
                    "200" | "203" | "204" => Some(status.green().to_string()),
                    _ => None,
                };

                if let Some(colored) = colored {
                    ligne_coloree = ligne_coloree.replacen(status, &colored, 1);
                }
            }

            if let Some(method) = captures.name("method") {
                let method = method.as_str();
                let colored = match method {
                    "GET" => Some(method.green().to_string()),
                    "POST" | "OPTIONS" => Some(method.blue().to_string()),
                    "PUT" => Some(method.yellow().to_string()),
                    "DELETE" => Some(method.red().to_string()),
                    "PATCH" => Some(method.truecolor(128, 128, 0).to_string()),
                    _ => None,
                };

                if let Some(colored) = colored {
                    ligne_coloree = (&*ligne_coloree.replacen(method, &colored, 1)).parse()?;
                }
            }
        }

        for regex in [re_custom, re_symfony,re_syslog] {
            
            if let Some(captures) = regex.captures(&line) {
                if let Some(level) = captures.name("level") {
                    let level = level.as_str();
                    let colored = match level {
                        "WARNING" => Some(level.yellow().to_string()),
                        "ERROR" | "CRITICAL" => Some(level.red().to_string()),
                        "SUCCESS" => Some(level.green().to_string()),
                        "INFO" => Some(level.blue().to_string()),
                        "DEBUG" => Some(level.cyan().to_string()),
                        _ => None,
                    };

                    if let Some(colored) = colored {
                        ligne_coloree = (&*ligne_coloree.replacen(level, &colored, 1)).parse()?;
                    }
                }
                
                if let Some(date) = captures.name("date") {
                    let date = date.as_str();
                    let colored = date.green().bold().to_string();
                    ligne_coloree = ligne_coloree.replacen(date, &colored, 1);
                }


            }
        }

        println!("{}", ligne_coloree);
    }

    Ok(())
}

