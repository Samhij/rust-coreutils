use clap::Parser;
use coreutils::GlobalOpts;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(
    name = "head",
    about = "Prints the first 10 lines of each FILE to standard output.",
    version
)]
struct HeadArgs {
    /// Prints the first NUM lines instead of the first 10
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    lines: usize,

    /// Files to read
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    #[command(flatten)]
    global: GlobalOpts,
}

fn main() {
    let args = HeadArgs::parse();
    let mut had_error = false;

    let files = if args.files.is_empty() {
        vec!["-".to_string()]
    } else {
        args.files
    };

    for file_path in &files {
        let reader: Box<dyn BufRead> = if file_path == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            match File::open(file_path) {
                Ok(file) => Box::new(BufReader::new(file)),
                Err(err) => {
                    eprintln!("head: {}: {}", file_path, err);
                    had_error = true;
                    continue;
                }
            }
        };

        for line in reader.lines().take(args.lines) {
            match line {
                Ok(l) => println!("{}", l),
                Err(err) => {
                    eprintln!("head: {}: Error reading line: {}", file_path, err);
                    had_error = true;
                    break;
                }
            }
        }
    }

    if had_error {
        process::exit(1);
    }
}
