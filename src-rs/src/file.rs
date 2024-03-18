use std::{fs, path::Path};

use eyre::{bail, Context, ContextCompat, Result};
use sqlx::{sqlite::SqliteConnectOptions, Row, SqlitePool};
use tokio::sync::RwLock;

use crate::wrap_errs;

const APPLICATION_NAME: &str = "chronochart";

macro_rules! get_pool {
    ($self:ident) => {
        match $self.pool.read().await.as_ref() {
            None => bail!("No project file is currently open"),
            Some(pool) => pool,
        }
    };
}

pub struct FileHandler {
    pool: RwLock<Option<SqlitePool>>,
}

impl FileHandler {
    #[inline]
    pub fn new() -> Self {
        return Self {
            pool: RwLock::new(None),
        };
    }

    pub async fn connect(&self, path: &str, create: bool) -> Result<()> {
        wrap_errs!(
            {
                if create && Path::new(path).is_file() {
                    fs::remove_file(path).wrap_err("Error replacing previous file")?;
                }

                let options = SqliteConnectOptions::new()
                    .create_if_missing(create)
                    .filename(path);
                let pool = SqlitePool::connect_with(options).await?;

                if !create && get_metadata(&pool, "application").await? != APPLICATION_NAME {
                    bail!("File is not a chronochart project file");
                }

                sqlx::migrate!()
                    .run(&pool)
                    .await
                    .wrap_err("Failed to execute database migrations")?;

                if create {
                    set_metadata(&pool, "application", APPLICATION_NAME).await?
                }

                // Close and replace previous pool
                let mut old = self.pool.write().await;
                match old.as_ref() {
                    Some(pool) => pool.close().await,
                    None => (),
                }

                old.replace(pool);
            },
            format!("Could not connect to file '{}'", path)
        )?;

        return Ok(());
    }

    pub async fn is_connected(&self) -> bool {
        return self.pool.read().await.is_some();
    }

    pub async fn get_metadata(&self, key: &str) -> Result<String> {
        return wrap_errs!(
            get_metadata(get_pool!(self), key).await?,
            format!("Could not get metadata '{}'", key)
        );
    }

    pub async fn set_metadata(&self, key: &str, value: &str) -> Result<()> {
        wrap_errs!(
            set_metadata(get_pool!(self), key, value).await?,
            format!("Error setting metadata '{}' to '{}'", key, value)
        )?;

        return Ok(());
    }
}

async fn set_metadata(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
    sqlx::query("INSERT OR REPLACE INTO [metadata] ([key], [value]) VALUES (?, ?)")
        .bind(key)
        .bind(value)
        .execute(pool)
        .await?;
    return Ok(());
}

async fn get_metadata(pool: &SqlitePool, key: &str) -> Result<String> {
    Ok(
        sqlx::query("SELECT [value] FROM [metadata] WHERE [key] = ?")
            .bind(key)
            .fetch_optional(pool)
            .await?
            .wrap_err_with(|| format!("No metadata with key '{}' exists", key))?
            .try_get(0)?,
    )
}
