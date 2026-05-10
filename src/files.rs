//! Fonctions d'acces fichier et d'affichage colore.
//!
//! Ce module centralise :
//! - la lecture des lignes de logs ;
//! - la conversion vers [`LogEntry`](crate::parser::LogEntry) ;
//! - la coloration terminale des champs detectes.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use crate::parser::LogEntry;
use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;

/// Lit un fichier de logs et retourne sa version parsee ligne par ligne.
///
/// Chaque ligne est convertie en [`LogEntry`] via [`LogEntry::parse`].
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

/// Affiche un flux de lignes en colorant les champs reconnus.
///
/// La fonction tente de colorer, selon le format detecte :
/// - la date ;
/// - le niveau (`INFO`, `ERROR`, etc.) ;
/// - la methode HTTP ;
/// - le status HTTP ;
/// - certains champs syslog comme l'hote ou le process.
///
/// Le `reader` est consomme jusqu'a la fin du flux.
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


/// Affiche une seule entree ou un bloc de texte deja filtre avec coloration.
///
/// Cette variante est utilisee apres application d'un filtre logique
/// sur des [`LogEntry`] deja parses.
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

