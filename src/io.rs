use std::io::{Read, Write};

/// Generic helper that copies bytes directly from any reader to standard output
pub fn read_and_print<R: Read>(mut reader: R) -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();
    std::io::copy(&mut reader, &mut stdout)?;
    stdout.flush()?;
    Ok(())
}

pub fn print_headers(file_path: &str) {
    let display_name = if file_path == "-" {
        "standard input"
    } else {
        file_path
    };
    println!("==> {} <==", display_name);
}
