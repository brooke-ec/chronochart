use eyre::{Context, Result};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

const APPLICATION_NAME: &str = "chronochart";

pub struct FileHandler {
    pool: Option<SqlitePool>,
}

impl FileHandler {
    #[inline]
    pub fn new() -> Self {
        return Self { pool: None };
    }

    pub async fn connect(&self, path: &str, create: bool) -> Result<()> {
        let options = SqliteConnectOptions::new()
            .create_if_missing(create)
            .filename(path);
        let pool = SqlitePool::connect_with(options).await?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .wrap_err("Failed to execute database migrations")?;

        return Ok(());
    }
}
