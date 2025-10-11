use std::{fs, path::Path};

use eyre::{bail, Context, ContextCompat, Result};
use sqlx::{query, query_as, query_scalar, sqlite::SqliteConnectOptions, Row, SqlitePool};
use tokio::sync::RwLock;

use crate::{util::GatherFutures, wrap_errs};

const APPLICATION_NAME: &str = "chronochart";

static POOL: RwLock<Option<SqlitePool>> = RwLock::const_new(None);

macro_rules! get_pool {
    () => {
        match POOL.read().await.as_ref() {
            None => bail!("No project file is currently open."),
            Some(pool) => pool,
        }
    };
}

pub async fn connect(path: &str, create: bool) -> Result<()> {
    wrap_errs!(
        {
            if create && Path::new(path).is_file() {
                fs::remove_file(path).wrap_err("There was an error removing the previous file.")?;
            }

            let options = SqliteConnectOptions::new().create_if_missing(create).filename(path);
            let pool = SqlitePool::connect_with(options).await?;

            if !create && get_metadata_raw(&pool, "application").await? != APPLICATION_NAME {
                bail!("File is not a chronochart project file.");
            }

            sqlx::migrate!()
                .run(&pool)
                .await
                .wrap_err("Failed to execute database migrations.")?;

            if create {
                set_metadata_raw(&pool, "application", APPLICATION_NAME).await?
            }

            // Close and replace previous pool
            let mut old = POOL.write().await;
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

pub async fn disconnect() {
    let mut pool = POOL.write().await;
    match pool.as_ref() {
        Some(pool) => pool.close().await,
        None => (),
    }

    pool.take();
}

pub async fn is_connected() -> bool {
    return POOL.read().await.is_some();
}

pub async fn get_metadata(key: &str) -> Result<String> {
    return wrap_errs!(
        get_metadata_raw(get_pool!(), key).await?,
        format!("Could not get metadata '{}'", key)
    );
}

pub async fn set_metadata(key: &str, value: &str) -> Result<()> {
    wrap_errs!(
        set_metadata_raw(get_pool!(), key, value).await?,
        format!("Error setting metadata '{}' to '{}'", key, value)
    )?;

    return Ok(());
}

pub async fn get_timelines() -> Result<Vec<crate::model::Timeline>> {
    Ok(query_as!(crate::model::Timeline, "SELECT * FROM [timeline]")
        .fetch_all(get_pool!())
        .await
        .wrap_err_with(|| format!("Could not get timelines"))?)
}

pub async fn get_events() -> Result<Vec<crate::model::Event>> {
    let rows = query!("SELECT * FROM [event] ORDER BY [timestamp] ASC")
        .fetch_all(get_pool!())
        .await
        .wrap_err_with(|| format!("Could not get events"))?;

    let result = rows
        .into_iter()
        .map(async move |r| {
            let timelines = query_scalar!("SELECT [timeline_uuid] FROM [event_timeline] WHERE [event_uuid] = ?", r.uuid)
                .fetch_all(get_pool!())
                .await
                .wrap_err_with(|| format!("Could not get timelines for event '{}'", r.uuid))?;

            Ok(crate::model::Event {
                uuid: r.uuid,
                timestamp: r.timestamp as i32,
                color: r.color,
                content: r.content,
                timelines,
            })
        })
        .gather()
        .await
        .into_iter()
        .collect::<Result<Vec<crate::model::Event>>>()?;

    return Ok(result);
}

async fn set_metadata_raw(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
    sqlx::query("INSERT OR REPLACE INTO [metadata] ([key], [value]) VALUES (?, ?)")
        .bind(key)
        .bind(value)
        .execute(pool)
        .await?;
    return Ok(());
}

async fn get_metadata_raw(pool: &SqlitePool, key: &str) -> Result<String> {
    Ok(sqlx::query("SELECT [value] FROM [metadata] WHERE [key] = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?
        .wrap_err_with(|| format!("No metadata with key '{}' exists", key))?
        .try_get(0)?)
}
