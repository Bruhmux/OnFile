use crate::cli;
use clap::Parser;
use dotenvy::dotenv;
use std::{env, net::Ipv4Addr, sync::Arc};
use tracing::info;

#[derive(Debug)]
struct ServerConfig {
    host: Ipv4Addr,
    port: u16,
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &Ipv4Addr {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

pub async fn init_config() -> Arc<Config> {
    dotenv().ok();
    let args = cli::options::Args::parse();

    let server_config = ServerConfig {
        host: env::var("HOST")
            .unwrap_or_else(|_| {
                info!("HOST not set in env... using cli options");
                args.addr
            })
            .parse()
            .expect("Must be a numerical Ipv4 address (127.0.0.1 default)"),
        port: env::var("PORT")
            .unwrap_or_else(|_| {
                info!("PORT not set in env... using cli options");
                args.port.to_string()
            })
            .parse()
            .expect("PORT must be a valid u16"),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Arc::new(Config {
        server: server_config,
        db: database_config,
    })
}
