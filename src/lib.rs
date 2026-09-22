pub mod lib {
    use clap::Args;
    use std::fmt::Display;
    use std::io;
    use std::io::{Read, Write};

    #[derive(Args, Debug)]
    pub struct GlobalOpts {
        /// Explain what is being done
        #[arg(short, long, default_value_t = false)]
        pub verbose: bool,
    }

    /// Generic helper that copies bytes directly from any reader to standard output
    pub fn read_and_print<R: Read>(mut reader: R) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        io::copy(&mut reader, &mut stdout)?;
        stdout.flush()?;
        Ok(())
    }

    pub fn print_headers(file_path: &String) {
        let display_name = if file_path == "-" {
            "standard input"
        } else {
            file_path
        };
        println!("==> {} <==", display_name);
    }

    pub trait CommandReport {
        fn name(&self) -> &'static str;

        fn report_error(&self, path: Option<&str>, err: impl Display) {
            match path {
                Some(p) => eprintln!("{}: {}: {}", self.name(), p, err),
                None => eprintln!("{}: {}", self.name(), err),
            }
        }
    }
}
