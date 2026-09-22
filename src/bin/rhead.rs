use clap::Parser;
use coreutils::lib::{CommandReport, GlobalOpts, print_headers};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(
    name = "rhead",
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

impl CommandReport for HeadArgs {
    fn name(&self) -> &'static str {
        "rhead"
    }
}

fn main() {
    let args = HeadArgs::parse();
    let mut had_error = false;

    let files = if args.files.is_empty() {
        vec!["-".to_string()]
    } else {
        args.files.clone()
    };

    let show_headers = files.len() > 1;

    for (i, file_path) in files.iter().enumerate() {
        let reader: Box<dyn BufRead> = if file_path == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            match File::open(file_path) {
                Ok(file) => Box::new(BufReader::new(file)),
                Err(err) => {
                    args.report_error(Some(file_path), err);
                    had_error = true;
                    continue;
                }
            }
        };

        if show_headers {
            if i > 0 {
                println!();
            }
            print_headers(file_path);
        }

        for line in reader.lines().take(args.lines) {
            match line {
                Ok(l) => println!("{}", l),
                Err(err) => {
                    args.report_error(Some(file_path), format!("Error reading line: {}", err));
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
