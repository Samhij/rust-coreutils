use clap::Parser;
use coreutils::lib::CommandReport;
use filetime::{FileTime, set_file_times};
use std::fs::OpenOptions;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "rtouch",
    about = "Change file access and modification times",
    version
)]
struct TouchArgs {
    /// Do not create any new files if they don't exist
    #[arg(short = 'c', long)]
    no_create: bool,

    #[arg(value_name = "FILE(S)", required = true)]
    files: Vec<String>,
}

impl CommandReport for TouchArgs {
    fn name(&self) -> &'static str {
        "rtouch"
    }
}

fn main() {
    let args = TouchArgs::parse();

    let mut had_error = false;

    for file_path in &args.files {
        match std::fs::exists(file_path) {
            Ok(true) => {
                let now = FileTime::now();
                if let Err(err) = set_file_times(file_path, now, now) {
                    args.report_error(Some(file_path), err);
                    had_error = true;
                }
            }
            Ok(false) => {
                if args.no_create {
                    continue;
                }

                if let Err(err) = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(file_path)
                {
                    args.report_error(Some(file_path), err);
                    had_error = true;
                }
            }
            Err(err) => {
                args.report_error(Some(file_path), err);
                had_error = true;
            }
        }
    }

    if had_error {
        process::exit(1);
    }
}
