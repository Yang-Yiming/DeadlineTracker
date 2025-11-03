use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json;
use ulid::Ulid;

use crate::persistence::repo::{HomeworkRepo, RepoError, RepoResult};
use crate::persistence::types::{HomeworkRecord, NewHomework, Patch};

pub struct SQLiteRepo {
    db_path: PathBuf,
    conn: Mutex<Connection>,
}

impl SQLiteRepo {
    pub fn new(dir: PathBuf) -> anyhow::Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let db_path = dir.join("deadlines.db");
        let conn = Connection::open(&db_path)?;
        // WAL mode for better safety/performance (best-effort)
        let _ = conn.pragma_update(None, "journal_mode", &"WAL");
        // foreign keys off (no FK here) but enabled doesn't hurt
        let _ = conn.pragma_update(None, "foreign_keys", &"ON");
        Self::init_schema(&conn)?;
        Ok(Self { db_path, conn: Mutex::new(conn) })
    }

    fn init_schema(conn: &Connection) -> anyhow::Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS homeworks (
                uid TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                due_text TEXT NOT NULL,
                difficulty INTEGER NOT NULL,
                progress INTEGER NOT NULL,
                tags_json TEXT NOT NULL,
                milestones_json TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                schema_version INTEGER NOT NULL DEFAULT 1
            );
            CREATE INDEX IF NOT EXISTS idx_homeworks_due_text ON homeworks(due_text);
            CREATE INDEX IF NOT EXISTS idx_homeworks_updated_at ON homeworks(updated_at);
            CREATE INDEX IF NOT EXISTS idx_homeworks_deleted ON homeworks(deleted);
            "#,
        )?;
        Ok(())
    }

    fn row_to_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<HomeworkRecord> {
        let tags_json: String = row.get("tags_json")?;
        let milestones_json: String = row.get("milestones_json")?;
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        let milestones: Vec<(u8, String)> = serde_json::from_str(&milestones_json).unwrap_or_default();
        Ok(HomeworkRecord {
            uid: row.get("uid")?,
            name: row.get("name")?,
            due_text: row.get("due_text")?,
            difficulty: row.get("difficulty")?,
            progress: row.get("progress")?,
            tags,
            milestones,
            deleted: row.get::<_, i64>("deleted")? != 0,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            schema_version: row.get("schema_version")?,
        })
    }

    fn record_to_columns(rec: &HomeworkRecord) -> (String, String) {
        let tags_json = serde_json::to_string(&rec.tags).unwrap_or("[]".into());
        let milestones_json = serde_json::to_string(&rec.milestones).unwrap_or("[]".into());
        (tags_json, milestones_json)
    }
}

impl HomeworkRepo for SQLiteRepo {
    fn list(&self) -> RepoResult<Vec<HomeworkRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM homeworks WHERE deleted=0 ORDER BY due_text ASC")
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| Self::row_to_record(row))
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| RepoError::Sql(e.to_string()))?);
        }
        Ok(out)
    }

    fn get(&self, uid: &str) -> RepoResult<Option<HomeworkRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM homeworks WHERE uid = ?1")
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        let rec: Option<HomeworkRecord> = stmt
            .query_row(params![uid], |row| Self::row_to_record(row))
            .optional()
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        Ok(rec)
    }

    fn create(&self, payload: NewHomework) -> RepoResult<HomeworkRecord> {
        let uid = Ulid::new().to_string();
        let now = Utc::now().timestamp();
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
        let (tags_json, milestones_json) = Self::record_to_columns(&rec);
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| RepoError::Sql(e.to_string()))?;
        tx.execute(
            r#"INSERT INTO homeworks
            (uid,name,due_text,difficulty,progress,tags_json,milestones_json,deleted,created_at,updated_at,schema_version)
            VALUES (?1,?2,?3,?4,?5,?6,?7,0,?8,?9,1)"#,
            params![
                rec.uid,
                rec.name,
                rec.due_text,
                rec.difficulty as i64,
                rec.progress as i64,
                tags_json,
                milestones_json,
                rec.created_at,
                rec.updated_at,
            ],
        )
        .map_err(|e| RepoError::Sql(e.to_string()))?;
        tx.commit().map_err(|e| RepoError::Sql(e.to_string()))?;
        Ok(rec)
    }

    fn update(&self, mut record: HomeworkRecord) -> RepoResult<HomeworkRecord> {
        record.updated_at = Utc::now().timestamp();
        let (tags_json, milestones_json) = Self::record_to_columns(&record);
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| RepoError::Sql(e.to_string()))?;
        let changed = tx
            .execute(
                r#"UPDATE homeworks SET
                name=?2,
                due_text=?3,
                difficulty=?4,
                progress=?5,
                tags_json=?6,
                milestones_json=?7,
                deleted=?8,
                updated_at=?9,
                schema_version=?10
                WHERE uid=?1"#,
                params![
                    record.uid,
                    record.name,
                    record.due_text,
                    record.difficulty as i64,
                    record.progress as i64,
                    tags_json,
                    milestones_json,
                    if record.deleted {1i64} else {0i64},
                    record.updated_at,
                    record.schema_version as i64,
                ],
            )
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        if changed == 0 { return Err(RepoError::NotFound); }
        tx.commit().map_err(|e| RepoError::Sql(e.to_string()))?;
        Ok(record)
    }

    fn patch(&self, uid: &str, patch: Patch) -> RepoResult<HomeworkRecord> {
        // load -> apply -> update
        let current = self.get(uid)?.ok_or(RepoError::NotFound)?;
        let now = Utc::now().timestamp();
        let mut merged = current.clone();
        merged.apply_patch(patch, now);
        self.update(merged.clone())?;
        Ok(merged)
    }

    fn delete(&self, uid: &str) -> RepoResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| RepoError::Sql(e.to_string()))?;
        let updated_at = Utc::now().timestamp();
        let changed = tx
            .execute(
                "UPDATE homeworks SET deleted=1, updated_at=?2 WHERE uid=?1",
                params![uid, updated_at],
            )
            .map_err(|e| RepoError::Sql(e.to_string()))?;
        if changed == 0 { return Err(RepoError::NotFound); }
        tx.commit().map_err(|e| RepoError::Sql(e.to_string()))?;
        Ok(())
    }
}
