use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Clone)]
pub struct Args {
    /// Specify target directory
    ///
    /// Defaults to `output/`.
    #[clap(short, long, default_value = "output")]
    pub target: PathBuf,

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
