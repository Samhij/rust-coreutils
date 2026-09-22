use clap::Args;

#[derive(Args, Debug)]
pub struct GlobalOpts {
    /// Explain what is being done
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
