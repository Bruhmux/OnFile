use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[clap(
    name = "Truths and Tombs",
    about = "A medevil multiplayer logical deduction game"
)]
pub struct Args {
    /// Set the listening address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub addr: String,

    /// Set the listening port
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
}
