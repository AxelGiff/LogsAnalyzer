mod files;
mod parser;

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

    let contents=files::read_file_contents(&args.logfile);
   
    let filter_type = &args.filter[0];
    let filter_value = &args.filter[1];
   
    let filtered: Vec<_>=contents.into_iter().filter(|line| line.matches_filter(filter_type,filter_value)).collect();
    if filtered.is_empty(){
        println!("Aucune ligne ne correspond au filtre.");
    }else {
        for entry in filtered {
            println!("{:?}", entry.raw);
        }
    }
}




