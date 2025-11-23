//! Persistence layer for DeadlineTracker (desktop-only for now)
//! Provides a repository abstraction with two implementations:
//! - MemoryRepo (when DIR=None; no persistence)
//! - SQLiteRepo (when DIR=Some(path); persisted to a SQLite file)

pub mod types;
pub mod repo;
pub mod memory;
pub mod json;

pub use repo::{init_repo, HomeworkRepo, RepoError};
pub use types::{HomeworkRecord, NewHomework, Patch};
