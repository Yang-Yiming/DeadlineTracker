use serde::{Deserialize, Serialize};

/// Minimal persisted record for a homework/deadline item.
/// Notes:
/// - `due_text`: formatted as "YYYY-MM-DD HH:MM" to align with existing Datetime.to_string()
/// - `tags_json` / `milestones_json` are kept as structured types here; adapters serialize to JSON TEXT for SQLite.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HomeworkRecord {
    pub uid: String,
    pub name: String,
    /// Due in text form "YYYY-MM-DD HH:MM"
    pub due_text: String,
    pub difficulty: u8,
    pub progress: u8,
    pub tags: Vec<String>,
    pub milestones: Vec<(u8, String)>,
    pub deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub schema_version: u16,
}

/// Creation payload without uid/timestamps. Repo will assign uid and timestamps.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NewHomework {
    pub name: String,
    /// "YYYY-MM-DD HH:MM"
    pub due_text: String,
    pub difficulty: u8,
    pub progress: u8,
    pub tags: Vec<String>,
    pub milestones: Vec<(u8, String)>,
}

/// Partial update. `None` means unchanged.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Patch {
    pub name: Option<String>,
    pub due_text: Option<String>,
    pub difficulty: Option<u8>,
    pub progress: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub milestones: Option<Vec<(u8, String)>>,
    pub deleted: Option<bool>,
}

impl HomeworkRecord {
    pub fn apply_patch(&mut self, patch: Patch, now_ts: i64) {
        if let Some(v) = patch.name { self.name = v; }
        if let Some(v) = patch.due_text { self.due_text = v; }
        if let Some(v) = patch.difficulty { self.difficulty = v; }
        if let Some(v) = patch.progress { self.progress = v; }
        if let Some(v) = patch.tags { self.tags = v; }
        if let Some(v) = patch.milestones { self.milestones = v; }
        if let Some(v) = patch.deleted { self.deleted = v; }
        self.updated_at = now_ts;
    }
}
