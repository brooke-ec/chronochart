use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
pub struct Timeline {
    pub uuid: String,
    pub title: String,
    pub color: String,
    pub parent_uuid: Option<String>,
}

pub struct RawEvent {
    pub uuid: String,
    pub timestamp: i64,
    pub color: String,
    pub content: String,
}

#[derive(Serialize, Type)]
pub struct Event {
    pub uuid: String,
    pub timestamp: i64,
    pub color: String,
    pub content: String,
    pub timelines: Vec<String>,
}
