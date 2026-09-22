use clap::Parser;
use coreutils::lib::CommandReport;
use std::path::PathBuf;
use std::{env, fs, process};

#[derive(Parser, Debug)]
#[command(name = "rpwd", about = "Output the current working directory", version)]
struct PwdArgs {
    /// Print the logical current working directory (default)
    #[arg(short = 'L', long, overrides_with = "physical")]
    logical: bool,

    /// Print the physical current working directory, avoiding all symlinks
    #[arg(short = 'P', long, overrides_with = "logical")]
    physical: bool,
}

impl CommandReport for PwdArgs {
    fn name(&self) -> &'static str {
        "rpwd"
    }
}

fn main() {
    let args = PwdArgs::parse();

    let current_dir = env::current_dir().unwrap_or_else(|err| {
        args.report_error(None, err);
        process::exit(1);
    });

    let print_physical = args.physical;

    let path = if print_physical {
        fs::canonicalize(&current_dir).unwrap_or_else(|_| current_dir)
    } else {
        if let Ok(env_pwd) = env::var("PWD") {
            let pwd_path = PathBuf::from(&env_pwd);

            if fs::canonicalize(&pwd_path).ok() == fs::canonicalize(&current_dir).ok() {
                pwd_path
            } else {
                current_dir
            }
        } else {
            current_dir
        }
    };

    println!("{}", path.display());
}
