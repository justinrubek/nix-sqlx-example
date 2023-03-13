use crate::{
    commands::{Commands, UserCommands},
    error::Result,
};
use clap::Parser;

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let args = commands::Args::parse();
    match args.command {
        Commands::User(hello) => {
            let cmd = hello.command;
            match cmd {
                UserCommands::List => {
                    let recs = sqlx::query!("SELECT id, name FROM users")
                        .fetch_all(&pool)
                        .await?;
                    tracing::info!("recs: {:?}", recs);
                }
                UserCommands::Add { name } => {
                    let rec =
                        sqlx::query!("INSERT INTO users (name) VALUES ($1) RETURNING id", name)
                            .fetch_one(&pool)
                            .await?;
                    tracing::info!("rec: {:?}", rec);
                }
            }
        }
    }

    Ok(())
}
