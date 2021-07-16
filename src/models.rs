use serde::Serialize;

#[derive(Serialize)]
pub struct Note {
    pub title: String,
    pub description: String,
}
