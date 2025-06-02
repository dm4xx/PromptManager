use sled::Db;
use anyhow::Result;
use std::path::PathBuf;

pub struct PromptDatabase {
    db: Db,
}

impl PromptDatabase {
    pub fn new(path: PathBuf) -> Result<Self> {
        let db = sled::open(path.as_path())?;
        Ok(Self { db })
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }
}
