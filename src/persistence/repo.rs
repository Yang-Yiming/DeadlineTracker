use std::path::PathBuf;
use std::sync::Arc;

use crate::persistence::memory::MemoryRepo;
use crate::persistence::sqlite::SQLiteRepo;
use crate::persistence::types::{HomeworkRecord, NewHomework, Patch};
use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;

/// Unified repository trait for local-only storage.
/// Implementations: MemoryRepo (DIR=None), SQLiteRepo (DIR=Some(path)).
pub trait HomeworkRepo: Send + Sync {
    fn list(&self) -> RepoResult<Vec<HomeworkRecord>>;
    fn get(&self, uid: &str) -> RepoResult<Option<HomeworkRecord>>;
    fn create(&self, payload: NewHomework) -> RepoResult<HomeworkRecord>;
    fn update(&self, record: HomeworkRecord) -> RepoResult<HomeworkRecord>;
    fn patch(&self, uid: &str, patch: Patch) -> RepoResult<HomeworkRecord>;
    fn delete(&self, uid: &str) -> RepoResult<()>; // soft delete
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("not found")] 
    NotFound,
    #[error("sqlite error: {0}")]
    Sql(String),
    #[error("serialization error: {0}")]
    Serde(String),
    #[error("unavailable: {0}")]
    Unavailable(String),
    #[error("unknown: {0}")]
    Unknown(String),
}

/// Initialize the repository based on data directory.
/// - None => MemoryRepo (no persistence)
/// - Some(path) => SQLiteRepo under that directory (creates file if missing)
pub fn init_repo(data_dir: Option<PathBuf>) -> RepoResult<Arc<dyn HomeworkRepo>> {
    match data_dir {
        None => Ok(Arc::new(MemoryRepo::new())),
        Some(path) => {
            let repo = SQLiteRepo::new(path).map_err(|e| RepoError::Unavailable(e.to_string()))?;
            Ok(Arc::new(repo))
        }
    }
}
