use std::{
    env::{self, var},
    fmt::Display,
};

use dotenv::dotenv;
use sqlx::{Pool, Postgres, Result, postgres::PgPoolOptions};

pub async fn init_connection(db_branch: Branch) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url(db_branch).as_str())
        .await?;

    Ok(pool)
}

fn get_db_url(db_branch: Branch) -> String {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    let general_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_url = format!("{general_url}{db_branch}");
    db_url
}

#[derive(Debug)]
pub enum Branch {
    Test,
    Staging,
    Production,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test => f.write_str("test"),
            Self::Staging => f.write_str("staging"),
            Self::Production => f.write_str("prod"),
        }
    }
}
