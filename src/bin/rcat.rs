use clap::Parser;
use coreutils::{GlobalOpts, read_and_print};
use std::fs::File;
use std::{io, process};

#[derive(Parser, Debug)]
#[command(
    name = "rcat",
    about = "Concatenate FILE(s) to standard output.",
    version
)]
struct CatArgs {
    /// Files to read
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    #[command(flatten)]
    global: GlobalOpts,
}

fn main() {
    let args = CatArgs::parse();
    let mut had_error = false;

    let files = if args.files.is_empty() {
        vec!["-".to_string()]
    } else {
        args.files
    };

    for file_path in &files {
        let result = if file_path == "-" {
            read_and_print(io::stdin().lock())
        } else {
            match File::open(file_path) {
                Ok(file) => read_and_print(file),
                Err(err) => Err(err),
            }
        };

        if let Err(err) = result {
            eprintln!("cat: {}: {}", file_path, err);
            had_error = true;
        }
    }

    if had_error {
        process::exit(1);
    }
}
