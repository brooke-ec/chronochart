fn main() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    println!("cargo:rerun-if-changed=migrations");
    tauri_build::build()
}
