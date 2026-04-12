use crate::cli;
use clap::Parser;
use dotenvy::dotenv;
use std::{env, sync::Arc};

#[derive(Debug)]
struct ServerConfig {
    host: String,
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

    pub fn server_host(&self) -> &str {
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
        host: env::var("HOST").unwrap_or_else(|_| args.addr),
        port: env::var("PORT")
            .expect("PORT not set set in env... using cli options")
            .parse::<u16>()
            .unwrap_or_else(|_| args.port),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Arc::new(Config {
        server: server_config,
        db: database_config,
    })
}
