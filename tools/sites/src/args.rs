use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(short, long)]
    pub serve: bool,
}

impl Args {
    /// Parse the command-line arguments
    ///
    /// Convenience method that saves the caller from having to import the
    /// `Clap` trait.
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}
