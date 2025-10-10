use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Error};

#[tokio::main]
async fn main() {
    // Create database for SQLX compile-time validation
    let db_path = "../example.db";
    let db_url = format!("sqlite://{}", db_path);

    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    async { Ok(sqlx::migrate!().run(&mut options.connect().await?).await?) }
        .await
        .unwrap_or_else(|e: Error| panic!("Failed to create development database: {}", e));

    println!("cargo:rerun-if-changed=migrations");
    tauri_build::build()
}
