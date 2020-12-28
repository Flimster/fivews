use std::io::{prelude::*, BufReader, BufWriter};
use std::fs;

use thiserror::Error;

use crate::entry::FiveWsEntry;
use crate::wal::WAL;
use crate::init::init_lidb;

pub type DbResult<T> = std::result::Result<T, DbError>;

const PAGE_SIZE: u64 = 4096 * 1024;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("failed to initialize database: `{0}`")]
    InitError(String),
    #[error("Lock was poisoned")]
    PoisonError,
    #[error("Unable to create checkpoint")]
    CheckpointError,
    #[error("failed to read from database")]
    ReadError,
    #[error("failed to write to database")]
    WriteError,
}

pub struct FiveWsDB {
    wal: WAL,
    db: Vec<FiveWsEntry>,
    path: String,
    checkpoint: usize,
}

// TODO: Move initialization of files to here
impl FiveWsDB {
    pub fn new(dir_path: &str) -> FiveWsDB {
        let checkpoint = init_lidb(dir_path);
        let log_location = format!("{}/log{}.lidb", dir_path, checkpoint);
        let checkpoint_location = format!("{}/checkpoint{}.lidb", dir_path, checkpoint);

        let wal = WAL::new(log_location);
        let logs = wal.get_logs();
        let db = init_in_memory_structure(checkpoint_location, logs).expect("Failed to initialize database");

        let path = dir_path.to_string();

        FiveWsDB { wal, db, path, checkpoint }
    }

    // Returns the size of the current wal log
    pub fn update<T: Into<String>>(&mut self, who: T, what: T, when: T, r#where: T, why: T) -> DbResult<()> {
        let entry = FiveWsEntry::new(who, what, when, r#where, why);
        let current_wal_size = self.wal.write(&entry).map_err(|_| DbError::WriteError)?;
        self.db.push(entry);

        if current_wal_size >= PAGE_SIZE {
            self.create_checkpoint().unwrap();
        }

        Ok(())

    }

    // TODO: Refactor this function
    pub fn create_checkpoint(&mut self) -> DbResult<()> {
        let entries = self.read("*");
        let f = fs::File::create("tmp").map_err(|_| DbError::CheckpointError)?;
        let mut writer = BufWriter::new(f);
        for e in entries {
            writer.write(e.to_string().as_bytes()).unwrap();
        }
        let new_checkpoint_file = format!("{}/checkpoint{}.lidb", self.path, self.checkpoint + 1);
        fs::rename("tmp", new_checkpoint_file).unwrap();
        fs::remove_file(format!("{}/checkpoint{}.lidb", self.path, self.checkpoint)).unwrap();

        let log_file_location = format!("{}/log{}.lidb", self.path, self.checkpoint + 1);
        let _ = fs::File::create(log_file_location).unwrap();
        fs::remove_file(format!("{}/log{}.lidb", self.path, self.checkpoint)).unwrap();

        let mut meta_file = fs::File::create("tmp").unwrap();
        let new_checkpoint = self.checkpoint + 1;
        meta_file.write(&new_checkpoint.to_be_bytes()).unwrap();
        fs::rename("tmp", format!("{}/meta", self.path)).unwrap();

        self.checkpoint += 1;

        Ok(())
    }

    pub fn read(&self, pattern: &str) -> Vec<FiveWsEntry> {
        self
            .db
            .iter()
            .filter(|x| {
                pattern == "*"
                    || x.like("who", pattern)
                    || x.like("what", pattern)
                    || x.like("when", pattern)
                    || x.like("where", pattern)
                    || x.like("why", pattern)
            })
            .map(|x| x.clone())
            .collect()
    }
}

fn init_in_memory_structure( checkpoint_location: String, logs: Vec<(String, FiveWsEntry)>,) -> DbResult<Vec<FiveWsEntry>> {
    let mut db = Vec::new();
    let checkpoint_file = std::fs::File::open(checkpoint_location).map_err(|e| DbError::InitError(e.to_string()))?;
    let reader = BufReader::new(checkpoint_file);

    // Initalizing from checkpoint file
    for l in reader.lines() {
        let l = l.map_err(|e| DbError::InitError(e.to_string()))?;
        let vec: Vec<&str> = l.split("|").collect();
        db.push(FiveWsEntry::from(vec));
    }

    // Intializing from logs
    for (op, entry) in logs {
        if op == "WRITE" {
            db.push(entry);
        }
    }

    Ok(db)
}
