use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Utc;
use ulid::Ulid;

use crate::persistence::repo::{HomeworkRepo, RepoError, RepoResult};
use crate::persistence::types::{HomeworkRecord, NewHomework, Patch};

pub struct JsonRepo {
    file_path: PathBuf,
    // Simple lock to serialize access to the file
    lock: Mutex<()>,
}

impl JsonRepo {
    pub fn new(dir: PathBuf) -> anyhow::Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = dir.join("deadlines.json");
        if !file_path.exists() {
            let file = File::create(&file_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer(writer, &Vec::<HomeworkRecord>::new())?;
        }
        Ok(Self {
            file_path,
            lock: Mutex::new(()),
        })
    }

    fn load(&self) -> RepoResult<Vec<HomeworkRecord>> {
        let file = File::open(&self.file_path).map_err(|e| RepoError::Unavailable(e.to_string()))?;
        let reader = BufReader::new(file);
        // If file is empty or invalid, return empty list or error. 
        // Here we assume valid JSON array.
        let records: Vec<HomeworkRecord> = serde_json::from_reader(reader).unwrap_or_default();
        Ok(records)
    }

    fn save(&self, records: &[HomeworkRecord]) -> RepoResult<()> {
        let file = File::create(&self.file_path).map_err(|e| RepoError::Unavailable(e.to_string()))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, records).map_err(|e| RepoError::Serde(e.to_string()))?;
        Ok(())
    }
}

impl HomeworkRepo for JsonRepo {
    fn list(&self) -> RepoResult<Vec<HomeworkRecord>> {
        let _guard = self.lock.lock().unwrap();
        let records = self.load()?;
        // Filter out deleted ones
        Ok(records.into_iter().filter(|r| !r.deleted).collect())
    }

    fn get(&self, uid: &str) -> RepoResult<Option<HomeworkRecord>> {
        let _guard = self.lock.lock().unwrap();
        let records = self.load()?;
        Ok(records.into_iter().find(|r| r.uid == uid))
    }

    fn create(&self, payload: NewHomework) -> RepoResult<HomeworkRecord> {
        let _guard = self.lock.lock().unwrap();
        let mut records = self.load()?;
        
        let now = Utc::now().timestamp();
        let rec = HomeworkRecord {
            uid: Ulid::new().to_string(),
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

        records.push(rec.clone());
        self.save(&records)?;
        Ok(rec)
    }

    fn update(&self, mut record: HomeworkRecord) -> RepoResult<HomeworkRecord> {
        let _guard = self.lock.lock().unwrap();
        let mut records = self.load()?;
        
        if let Some(idx) = records.iter().position(|r| r.uid == record.uid) {
            record.updated_at = Utc::now().timestamp();
            records[idx] = record.clone();
            self.save(&records)?;
            Ok(record)
        } else {
            Err(RepoError::NotFound)
        }
    }

    fn patch(&self, uid: &str, patch: Patch) -> RepoResult<HomeworkRecord> {
        let _guard = self.lock.lock().unwrap();
        let mut records = self.load()?;
        
        if let Some(idx) = records.iter().position(|r| r.uid == uid) {
            let mut current = records[idx].clone();
            let now = Utc::now().timestamp();
            current.apply_patch(patch, now);
            records[idx] = current.clone();
            self.save(&records)?;
            Ok(current)
        } else {
            Err(RepoError::NotFound)
        }
    }

    fn delete(&self, uid: &str) -> RepoResult<()> {
        let _guard = self.lock.lock().unwrap();
        let mut records = self.load()?;
        
        if let Some(idx) = records.iter().position(|r| r.uid == uid) {
            records[idx].deleted = true;
            records[idx].updated_at = Utc::now().timestamp();
            self.save(&records)?;
            Ok(())
        } else {
            Err(RepoError::NotFound)
        }
    }
}
