mod files;
mod parser;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use crate::parser::{RE_CUSTOM, RE_SYSLOG};
use crate::parser::RE_HTTP_METHOD;
use crate::parser::RE_SYMFONY;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    logfile: PathBuf, // Faut faire --logfile "axel"

    #[arg(short,long,num_args = 2, value_names = ["FIELD", "VALUE"])]
    filter: Vec<String>, // Faut faire --filter "axel"
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let contents = match files::read_file_contents(&args.logfile) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Erreur lecture fichier: {e}");
            return Ok(());
        }
    };
    let re_http_method = &RE_HTTP_METHOD;
    let re_custom_ssh = &RE_CUSTOM;
    let re_custom_symfony = &RE_SYMFONY;
    let re_custom_syslog = &RE_SYSLOG;


    if args.filter.is_empty(){
      
        let contenu = File::open(&args.logfile).unwrap();

        let mut reader = BufReader::new(contenu);
        files::print_colored_lines(&mut reader,&re_http_method ,&re_custom_ssh, &re_custom_symfony,&re_custom_syslog).expect("TODO: panic message");


        /*for ligne in reader.lines() {
            let line = ligne?;
            let mut ligne_coloree = line.clone();
            if let Some(captures) = re_http_method.captures(&line) {
                if let Some(status) = captures.name("status") {
                    let status = status.as_str();

                    let status_colore = match status {
                        "404" | "500" | "503" | "403" => Some(status.red().to_string()),
                        "200" | "203" | "204" => Some(status.green().to_string()),
                        _ => None,
                    };

                    if let Some(colored) = status_colore {
                        ligne_coloree = ligne_coloree.replacen(status, &colored, 1);
                    }
                }

                if let Some(method) = captures.name("method") {
                    let method = method.as_str();

                    let method_colore = match method {
                        "GET" => Some(method.green().to_string()),
                        "POST" | "OPTIONS" => Some(method.blue().to_string()),
                        "PUT" => Some(method.yellow().to_string()),
                        "DELETE" => Some(method.red().to_string()),
                        "PATCH" => Some(method.truecolor(128, 128, 0).to_string()),
                        _ => None,
                    };

                    if let Some(colored) = method_colore {
                        ligne_coloree = ligne_coloree.replacen(method, &colored, 1);
                    }
                }

                println!("{}", ligne_coloree);
            }
            if let Some(captures) = re_custom_ssh.captures(&line) {

                if let Some(level) = captures.name("level") {
                    let level = level.as_str();

                    let level_colore = match level {
                       "WARNING" => Some(level.yellow().to_string()),
                        "ERROR" => Some(level.red().to_string()),
                        "SUCCESS" => Some(level.green().to_string()),
                        "INFO" => Some(level.blue().to_string()),
                        "DEBUG" => Some(level.cyan().to_string()),


                        _ => None,
                    };

                    if let Some(colored) = level_colore {
                        ligne_coloree = ligne_coloree.replacen(level, &colored, 1);
                    }
                }


            }
            if let Some(captures) = re_custom_symfony.captures(&line) {

                if let Some(level) = captures.name("level") {
                    let level = level.as_str();

                    let level_colore = match level {
                        "WARNING" => Some(level.yellow().to_string()),
                        "ERROR" => Some(level.red().to_string()),
                        "SUCCESS" => Some(level.green().to_string()),
                        "INFO" => Some(level.blue().to_string()),
                        "DEBUG" => Some(level.cyan().to_string()),
                        "CRITICAL" => Some(level.red().to_string()),


                        _ => None,
                    };

                    if let Some(colored) = level_colore {
                        ligne_coloree = ligne_coloree.replacen(level, &colored, 1);
                    }
                }


            }
                println!("{}", ligne_coloree);


        }  */

    }else {
        let filter_type = &args.filter[0];
        let filter_value = &args.filter[1];

        let filtered: Vec<_> = contents
            .into_iter()
            .filter(|line| line.matches_filter(filter_type, filter_value))
            .collect();

        if filtered.is_empty() {
            println!("Aucune ligne ne correspond au filtre.");
        } else {
            for entry in filtered {
                files::print_filtered_colored_lines(entry.raw,&re_http_method ,&re_custom_ssh, &re_custom_symfony,&re_custom_syslog).expect("TODO: panic message");

                // println!("{}", entry.raw);
            }
        }
    }
    Ok(())
}




