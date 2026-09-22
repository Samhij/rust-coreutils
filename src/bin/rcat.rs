use clap::Parser;
use coreutils::{CommandReport, GlobalOpts, read_and_print};
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

impl CommandReport for CatArgs {
    fn name(&self) -> &'static str {
        "rcat"
    }
}

fn main() {
    let args = CatArgs::parse();
    let mut had_error = false;

    let files = if args.files.is_empty() {
        vec!["-".to_string()]
    } else {
        args.files.clone()
    };

    for file_path in &files {
        let result = if file_path == "-" {
            if args.global.verbose {
                println!("Concatenating from stdin...");
            }
            read_and_print(io::stdin().lock())
        } else {
            match File::open(file_path) {
                Ok(file) => {
                    if args.global.verbose {
                        println!("Concatenating file: {}", file_path);
                    };
                    read_and_print(file)
                }
                Err(err) => Err(err),
            }
        };

        if let Err(err) = result {
            args.report_error(Some(file_path), err);
            had_error = true;
        }
    }

    if had_error {
        process::exit(1);
    }
}
