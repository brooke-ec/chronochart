use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
pub struct Timeline {
    pub uuid: String,
    pub title: String,
    pub color: String,
}
