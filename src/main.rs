mod files;
mod parser;

use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use clap::Parser;
use crate::parser::{RE_CUSTOM, RE_SYSLOG};
use crate::parser::RE_HTTP_METHOD;
use crate::parser::RE_SYMFONY;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    logfile: PathBuf, // Faut faire --logfile "axel"

    #[arg(short,long,num_args = 2, value_names = ["FIELD", "VALUE"])]
    filter: Vec<String>, // Faut faire --filter "axel"

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    tail: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let _ = match files::read_file_contents(&args.logfile) {
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
        let contenu = File::open(&args.logfile)?;
        let mut reader = BufReader::new(contenu);
        files::print_colored_lines(&mut reader, re_http_method, re_custom_ssh, re_custom_symfony, re_custom_syslog)
            .expect("TODO: panic message");

        if args.tail {
            let (tx, rx) = mpsc::channel::<Result<Event>>();
            let mut watcher = notify::recommended_watcher(tx)?;
            watcher.watch(Path::new(&args.logfile), RecursiveMode::NonRecursive)?;

            for res in &rx {
                match res {
                    Ok(_event) => {
                        let contenu = File::open(&args.logfile)?;
                        let mut reader = BufReader::new(contenu);
                        files::print_colored_lines(
                            &mut reader,
                            re_http_method,
                            re_custom_ssh,
                            re_custom_symfony,
                            re_custom_syslog,
                        ).expect("TODO: panic message");
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        }
    }else {
        let filter_type = &args.filter[0];
        let filter_value = &args.filter[1];

        let print_filtered = || -> std::result::Result<(), Box<dyn std::error::Error>> {
            let contents = files::read_file_contents(&args.logfile)?;

            let filtered: Vec<_> = contents
                .into_iter()
                .filter(|line| line.matches_filter(filter_type, filter_value))
                .collect();

            if filtered.is_empty() {
                println!("Aucune ligne ne correspond au filtre.");
            }
            else {
                for entry in &filtered {
                    files::print_filtered_colored_lines(
                        &entry.raw,
                        re_http_method,
                        re_custom_ssh,
                        re_custom_symfony,
                        re_custom_syslog,
                    )?;
                }

        }
            Ok(())
    };
        print_filtered().unwrap();
        if args.tail {
            let (tx, rx) = mpsc::channel::<Result<Event>>();
            let mut watcher = notify::recommended_watcher(tx)?;
            watcher.watch(Path::new(&args.logfile), RecursiveMode::NonRecursive)?;

            for res in &rx {
                match res {
                    Ok(_) => {
                        print_filtered().unwrap();
                    }
                    Err(e) => eprintln!("watch error: {:?}", e),
                }
            }
        }
    }

    Ok(())
}




