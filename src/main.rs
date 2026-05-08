use std::path::PathBuf;
use clap::Parser;
use std::fs;

use clap::{ArgAction, Command, arg, command, value_parser};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: PathBuf, // Faut faire --file "axel"
}

fn main() {
    let args = Args::parse();




     match read_file_contents(&args.file){
        Ok(contents) =>println!("{}",contents),
        Err(error) => println!("{}",error),
    };
}

fn read_file_contents(path: &PathBuf) -> Result<String, std::io::Error> {

    fs::read_to_string(path)
   

}


