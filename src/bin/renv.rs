use clap::Parser;
use coreutils::CommandReport;
use std::collections::HashMap;
use std::process::Command;
use std::{env, process};

#[derive(Parser, Debug)]
#[command(name = "renv", about = "Set environment and run command", version)]
struct EnvArgs {
    #[arg(short, long)]
    ignore_environment: bool,

    #[arg(short, long, value_name = "NAME")]
    unset: Vec<String>,

    rest: Vec<String>,
}

impl CommandReport for EnvArgs {
    fn name(&self) -> &'static str {
        "renv"
    }
}

fn main() {
    let args = EnvArgs::parse();
    let (custom_vars, command, cmd_args) = parse_rest(args.rest.clone());

    if command.is_none() {
        let mut env_map: HashMap<String, String> = if args.ignore_environment {
            HashMap::new()
        } else {
            env::vars().collect()
        };

        for name in &args.unset {
            env_map.remove(name);
        }

        for (k, v) in custom_vars {
            env_map.insert(k, v);
        }

        let mut keys: Vec<_> = env_map.keys().cloned().collect();
        keys.sort();
        for k in keys {
            println!("{}={}", k, env_map[&k]);
        }

        return;
    }

    let cmd_name = command.unwrap();
    let mut cmd = Command::new(&cmd_name);

    if args.ignore_environment {
        cmd.env_clear();
    }

    for name in &args.unset {
        cmd.env_remove(name);
    }

    cmd.envs(custom_vars);
    cmd.args(cmd_args);

    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                process::exit(code);
            }
        },
        Err (err) => {
            args.report_error(Some(&cmd_name), err);
            process::exit(127);
        }
    }
}

fn parse_rest(rest: Vec<String>) -> (HashMap<String, String>, Option<String>, Vec<String>) {
    let mut vars = HashMap::new();
    let mut iter = rest.into_iter();
    let mut command = None;
    let mut args = Vec::new();

    while let Some(arg) = iter.next() {
        if let Some((key, val)) = arg.split_once('=') {
            vars.insert(key.to_string(), val.to_string());
        } else {
            command = Some(arg);
            args.extend(iter);
            break;
        }
    }

    (vars, command, args)
}