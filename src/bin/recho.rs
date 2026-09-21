use clap::Parser;
use coreutils::lib::GlobalOpts;

#[derive(Parser, Debug)]
#[command(
    name = "recho",
    about = "Write arguments to the standard output",
    version
)]
struct EchoArgs {
    #[arg(value_name = "STRING")]
    string: Vec<String>,

    /// Do not print a trailing newline character
    #[arg(short)]
    no_newline: bool,

    #[command(flatten)]
    global: GlobalOpts,
}

fn main() {
    let args = EchoArgs::parse();
    let string = args.string.join(" ");

    if args.no_newline {
        print!("{}", string);
    } else {
        println!("{}", string);
    }
}
