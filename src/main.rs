mod files;
mod parser;

use std::fs;
use std::path::PathBuf;
use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    logfile: PathBuf, // Faut faire --logfile "axel"

    #[arg(long,num_args = 2, value_names = ["FIELD", "VALUE"])]
    filter: Vec<String>, // Faut faire --filter "axel"
}

fn main() {
    let args = Args::parse();

    let contents = match files::read_file_contents(&args.logfile) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Erreur lecture fichier: {e}");
            return;
        }
    };
    if args.filter.is_empty(){
        let contenu = fs::read_to_string(&args.logfile).expect("Un problème a eu lieu sur la lecture du fichier");
        println!("{}", contenu);
    }else {
        let filter_type = &args.filter[0];
        let filter_value = &args.filter[1];
        if filter_value == "" {
            println!("VIDE");
        }
        let filtered: Vec<_> = contents
            .into_iter()
            .filter(|line| line.matches_filter(filter_type, filter_value))
            .collect();

        if filtered.is_empty() {
            println!("Aucune ligne ne correspond au filtre.");
        } else {
            for entry in filtered {
                println!("{}", entry.raw);
            }
        }
    }
}




