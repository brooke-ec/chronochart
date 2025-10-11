use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Error};

#[tokio::main]
async fn main() {
    create_db().await;
    println!("cargo:rerun-if-changed=migrations");
    tauri_build::build()
}

// Create database for SQLX compile-time validation
async fn create_db() {
    let db_path = "../example.cro";
    let db_url = format!("sqlite://{}", db_path);

    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    let options = SqliteConnectOptions::new().filename(db_path).create_if_missing(true);

    for _ in 0..=1 {
        if let Err::<(), Error>(e) = async { Ok(sqlx::migrate!().run(&mut options.connect().await?).await?) }.await {
            println!("cargo::warning=Database migration failed: {}, recreating", e);

            if let Err(e) = std::fs::remove_file(db_path) {
                println!("cargo::warning=Failed to remove database file: {}", e);
            }
        } else {
            return;
        }
    }

    println!("cargo::error=Failed to create development database after two attempts");
}
