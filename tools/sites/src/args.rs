use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(short, long)]
    pub serve: bool,
}
