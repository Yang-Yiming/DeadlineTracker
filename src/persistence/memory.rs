use std::collections::HashMap;
use std::sync::Mutex;

use chrono::Utc;
use ulid::Ulid;

use crate::persistence::repo::{HomeworkRepo, RepoError, RepoResult};
use crate::persistence::types::{HomeworkRecord, NewHomework, Patch};

pub struct MemoryRepo {
    // in-memory map keyed by uid
    inner: Mutex<HashMap<String, HomeworkRecord>>,
}

impl MemoryRepo {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
}

impl HomeworkRepo for MemoryRepo {
    fn list(&self) -> RepoResult<Vec<HomeworkRecord>> {
        let map = self.inner.lock().unwrap();
        let mut v: Vec<_> = map.values().cloned().collect();
        v.sort_by(|a, b| a.due_text.cmp(&b.due_text));
        Ok(v)
    }

    fn get(&self, uid: &str) -> RepoResult<Option<HomeworkRecord>> {
        let map = self.inner.lock().unwrap();
        Ok(map.get(uid).cloned())
    }

    fn create(&self, payload: NewHomework) -> RepoResult<HomeworkRecord> {
        let mut map = self.inner.lock().unwrap();
        let now = Utc::now().timestamp();
        let uid = Ulid::new().to_string();
        let rec = HomeworkRecord {
            uid: uid.clone(),
            name: payload.name,
            due_text: payload.due_text,
            difficulty: payload.difficulty,
            progress: payload.progress,
            tags: payload.tags,
            milestones: payload.milestones,
            deleted: false,
            created_at: now,
            updated_at: now,
            schema_version: 1,
        };
        map.insert(uid.clone(), rec.clone());
        Ok(rec)
    }

    fn update(&self, record: HomeworkRecord) -> RepoResult<HomeworkRecord> {
        let mut map = self.inner.lock().unwrap();
        if !map.contains_key(&record.uid) {
            return Err(RepoError::NotFound);
        }
        let mut record = record;
        record.updated_at = Utc::now().timestamp();
        map.insert(record.uid.clone(), record.clone());
        Ok(record)
    }

    fn patch(&self, uid: &str, patch: Patch) -> RepoResult<HomeworkRecord> {
        let mut map = self.inner.lock().unwrap();
        let rec = map.get_mut(uid).ok_or(RepoError::NotFound)?;
        let now = Utc::now().timestamp();
        let mut cloned = rec.clone();
        cloned.apply_patch(patch, now);
        *rec = cloned.clone();
        Ok(cloned)
    }

    fn delete(&self, uid: &str) -> RepoResult<()> {
        let mut map = self.inner.lock().unwrap();
        let rec = map.get_mut(uid).ok_or(RepoError::NotFound)?;
        rec.deleted = true;
        rec.updated_at = Utc::now().timestamp();
        Ok(())
    }
}
