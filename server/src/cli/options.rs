use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "ExFolio", about = "A portfolio analysis tool")]
pub struct Opt {
    /// Set the listening address
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    pub addr: String,

    /// Set the listening port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}
