use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
pub struct Timeline {
    pub uuid: String,
    pub title: String,
    pub color: String,
}

#[derive(Serialize, Type)]
pub struct Event {
    pub uuid: String,
    pub timestamp: i32,
    pub color: String,
    pub title: String,
    pub timelines: Vec<String>,
}
