use chrono::{DateTime, Local};
use clap::Parser;
use coreutils::GlobalOpts;
use std::ffi::CString;
use std::fs::DirEntry;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;
use std::{fs, process};

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
                print_long_format(&entry);
            } else {
                print!("{}  ", file_name.display());
            }
        }
    }

    if !args.list {
        println!();
    }

    if had_error {
        process::exit(1);
    }
}

fn print_long_format(entry: &DirEntry) {
    let metadata = match entry.metadata() {
        Ok(m) => m,
        Err(_) => return,
    };

    // File Type & Permission String (e.g., -rw-r--r--)
    let mode = metadata.permissions().mode();
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    let has_xattr = check_xattr(entry);
    let permissions = format!(
        "{}{}",
        parse_permissions(file_type, mode),
        if has_xattr { "@" } else { " " }
    );

    // Number of Hard Links
    let nlink = metadata.nlink();

    // Owner User and Group (Resolves UID/GID to actual names if `users` crate is used,
    // otherwise falls back to displaying the raw IDs)
    let uid = metadata.uid();
    let gid = metadata.gid();
    let user_name = users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());
    let group_name = users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string());

    // Size in Bytes
    let size = metadata.len();

    // Formatted Modified Time (e.g., Sep 20 23:30)
    let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modified_time.into();
    let formatted_time = datetime.format("%b %e %H:%M").to_string();

    // File Name
    let file_name = entry.file_name();

    println!(
        "{} {:>2} {:<4} {:<4} {:>4} {} {}",
        permissions,
        nlink,
        user_name,
        group_name,
        size,
        formatted_time,
        file_name.display()
    );
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
    #[cfg(target_os = "macos")]
    {
        if let Some(path_str) = entry.path().to_str() {
            if let Ok(c_path) = CString::new(path_str) {
                let size = unsafe { libc::listxattr(c_path.as_ptr(), std::ptr::null_mut(), 0, 0) };
                return size > 0;
            }
        }
    }
    false
}
