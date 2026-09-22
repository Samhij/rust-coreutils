use clap::Parser;
use coreutils::{CommandReport, GlobalOpts};
use std::io::{self, ErrorKind, Write};
use std::process;

#[derive(Parser, Debug)]
#[command(name = "ryes", about = "Be repetitively affirmative", version)]
struct YesArgs {
    #[arg(value_name = "EXPLETIVE")]
    expletive: Vec<String>,

    #[command(flatten)]
    global: GlobalOpts,
}

impl CommandReport for YesArgs {
    fn name(&self) -> &'static str {
        "ryes"
    }
}

fn main() {
    let args = YesArgs::parse();

    let to_print = if args.expletive.is_empty() {
        String::from("y")
    } else {
        args.expletive.join(" ")
    };

    let mut stdout = io::stdout().lock();
    loop {
        if let Err(err) = writeln!(stdout, "{}", to_print) {
            if err.kind() == ErrorKind::BrokenPipe {
                process::exit(0);
            }
            args.report_error(None, err);
            process::exit(1);
        }
        stdout.flush().unwrap();
    }
}
