use clap::Args;
use std::io;
use std::io::{Read, Write};

#[derive(Args, Debug)]
pub struct GlobalOpts {
    /// Explain what is being done
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

/// Generic helper that copies bytes directly from any reader to standard output
pub fn read_and_print<R: Read>(mut reader: R) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    io::copy(&mut reader, &mut stdout)?;
    stdout.flush()?;
    Ok(())
}
