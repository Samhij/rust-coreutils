use chrono::{DateTime, Local};
use clap::Parser;
use coreutils::lib::GlobalOpts;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;
use std::{fs, process};

struct LongFormatEntry {
    permissions: String,
    nlink: u64,
    user_name: String,
    group_name: String,
    size: u64,
    formatted_time: String,
    file_name: OsString,
}

#[derive(Parser, Debug)]
#[command(name = "rls", about = "List directory contents", version)]
struct LsArgs {
    /// Show hidden files and directories
    #[arg(short, long)]
    all: bool,

    /// Show each file or directory on a separate line
    #[arg(short, long)]
    list: bool,

    /// Path to directory
    #[arg(value_name = "PATH")]
    paths: Vec<String>,

    #[command(flatten)]
    global: GlobalOpts,
}
fn main() {
    let args = LsArgs::parse();
    let mut had_error = false;

    let paths = if args.paths.is_empty() {
        vec![".".to_string()]
    } else {
        args.paths
    };

    for path in &paths {
        let dir = match fs::read_dir(path) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("rls: {}: {}", path, err);
                had_error = true;
                continue;
            }
        };

        let mut long_entries = Vec::new();

        for file in dir {
            let entry = match file {
                Ok(e) => e,
                Err(_) => continue,
            };

            let file_name = entry.file_name();
            let is_hidden = file_name.to_str().map_or(false, |s| s.starts_with('.'));

            if is_hidden && !args.all {
                continue;
            }

            if args.list {
                if let Some(long_entry) = collect_long_format(&entry) {
                    long_entries.push(long_entry);
                }
            } else {
                print!("{}  ", file_name.display());
            }
        }

        if args.list {
            print_long_format(&long_entries);
        }
    }

    if !args.list {
        println!();
    }

    if had_error {
        process::exit(1);
    }
}

fn collect_long_format(entry: &DirEntry) -> Option<LongFormatEntry> {
    let metadata = entry.metadata().ok()?;

    let mode = metadata.permissions().mode();
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    let mut permissions = parse_permissions(file_type, mode);
    if check_xattr(entry) {
        permissions.push('@');
    }

    let uid = metadata.uid();
    let gid = metadata.gid();
    let user_name = users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());
    let group_name = users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string());

    let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modified_time.into();
    let formatted_time = datetime.format("%b %e %H:%M").to_string();

    Some(LongFormatEntry {
        permissions,
        nlink: metadata.nlink(),
        user_name,
        group_name,
        size: metadata.len(),
        formatted_time,
        file_name: entry.file_name(),
    })
}

fn print_long_format(entries: &[LongFormatEntry]) {
    let nlink_width = entries
        .iter()
        .map(|e| e.nlink.to_string().len())
        .max()
        .unwrap_or(1);
    let user_width = entries.iter().map(|e| e.user_name.len()).max().unwrap_or(1);
    let group_width = entries
        .iter()
        .map(|e| e.group_name.len())
        .max()
        .unwrap_or(1);
    let size_width = entries
        .iter()
        .map(|e| e.size.to_string().len())
        .max()
        .unwrap_or(1);

    for entry in entries {
        println!(
            "{} {:nlink_width$} {:user_width$} {:group_width$} {:size_width$} {} {}",
            entry.permissions,
            entry.nlink,
            entry.user_name,
            entry.group_name,
            entry.size,
            entry.formatted_time,
            entry.file_name.display(),
            nlink_width = nlink_width,
            user_width = user_width,
            group_width = group_width,
            size_width = size_width,
        );
    }
}

fn parse_permissions(file_type: char, mode: u32) -> String {
    let mut p = String::with_capacity(10);
    p.push(file_type);

    let chars = ['r', 'w', 'x'];
    for i in (0..9).rev() {
        if (mode >> i) & 1 == 1 {
            p.push(chars[(8 - i) % 3]);
        } else {
            p.push('-');
        }
    }
    p
}

fn check_xattr(entry: &DirEntry) -> bool {
    xattr::list(entry.path())
        .map(|mut iter| iter.next().is_some())
        .unwrap_or(false)
}
