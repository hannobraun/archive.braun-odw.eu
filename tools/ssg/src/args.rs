use clap::Clap;

#[derive(Clap, Clone, Copy)]
pub struct Args {
    /// Enable dev mode
    ///
    /// Causes the sites tool to not only build the sites, but also serve them
    /// and rebuild them continuously as long as it is running.
    #[clap(short, long)]
    pub dev: bool,
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
