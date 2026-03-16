use anyhow::Context;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect_sqlite(database_url: &str) -> anyhow::Result<SqlitePool> {
    if let Some(parent) = database_url
        .strip_prefix("sqlite://")
        .and_then(|p| std::path::Path::new(p).parent())
    {
        tokio::fs::create_dir_all(parent)
            .await
            .context("create sqlite parent dir")?;
    }

    SqlitePoolOptions::new()
        .max_connections(16)
        .connect(database_url)
        .await
        .context("connect sqlite")
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("run migrations")
}
