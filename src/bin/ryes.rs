use clap::Parser;
use coreutils::lib::{CommandReport, GlobalOpts};
use std::io::{self, Write};

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
        writeln!(stdout, "{}", to_print).unwrap();
        stdout.flush().unwrap();
    }
}
