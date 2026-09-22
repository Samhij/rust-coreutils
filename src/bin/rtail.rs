use clap::Parser;
use coreutils::{CommandReport, GlobalOpts, print_headers};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(
    name = "rtail",
    about = "Prints the last 10 lines of each FILE to standard output.",
    version
)]
struct TailArgs {
    /// Prints the last NUM lines instead of the first 10
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    lines: usize,

    /// Files to read
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    #[command(flatten)]
    global: GlobalOpts,
}

impl CommandReport for TailArgs {
    fn name(&self) -> &'static str {
        "rtail"
    }
}

fn main() {
    let args = TailArgs::parse();
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

        let mut buffer = VecDeque::with_capacity(args.lines);
        let mut read_failed = false;

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if args.lines > 0 {
                        if buffer.len() == args.lines {
                            buffer.pop_front();
                        }
                        buffer.push_back(l);
                    }
                }

                Err(err) => {
                    args.report_error(Some(file_path), format!("Error reading line: {}", err));
                    had_error = true;
                    read_failed = true;
                    break;
                }
            }
        }

        if !read_failed {
            for line in buffer {
                println!("{}", line);
            }
        }
    }

    if had_error {
        process::exit(1);
    }
}
