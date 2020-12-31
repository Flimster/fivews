use std::fs;
use std::io::{prelude::*, BufReader, BufWriter};

use thiserror::Error;

use crate::entry::FiveWsEntry;
use crate::init::init_lidb;
use crate::wal::WAL;

pub type DbResult<T> = std::result::Result<T, DbError>;

const PAGE_SIZE: u64 = 4096;

///
#[derive(Error, Debug)]
pub enum DbError {
    #[error("failed to initialize database: `{0}`")]
    InitError(String),
    #[error("lock was poisoned")]
    PoisonError,
    #[error("unable to create new checkpoint")]
    CheckpointError,
    #[error("failed to read from database")]
    ReadError,
    #[error("failed to write to database")]
    WriteError,
}

pub struct FiveWsDB {
    wal: WAL,
    storage: Vec<FiveWsEntry>,
    path: String,
    checkpoint: usize,
}

impl FiveWsDB {
    /// Returns a FiveWsDB instance
    ///
    /// # Panics
    ///
    /// The function will panic if the given `dir_path` argument is not a valid path
    ///
    ///  # Examples
    ///
    /// ```
    /// use fivewsdb::db::FiveWsDB;
    ///
    /// let db = FiveWsDB::new("./db_path");
    /// ```
    ///
    pub fn new(dir_path: &str) -> FiveWsDB {
        let checkpoint = init_lidb(dir_path);
        let checkpoint_file_path = format!("{}/checkpoint{}.lidb", dir_path, checkpoint);
        let mut storage =
            init_from_checkpoint(checkpoint_file_path).expect("Failed to initialize database");

        let log_location = format!("{}/log{}.lidb", dir_path, checkpoint);

        let wal = WAL::new(log_location);

        let log_entries: Vec<FiveWsEntry> = wal
            .get_logs()
            .iter()
            .map(|entry| entry.to_owned())
            .collect();
        storage.extend(log_entries);

        let path = dir_path.to_string();

        FiveWsDB {
            wal,
            storage,
            path,
            checkpoint,
        }
    }

    // TODO: Neds a much better documentation
    /// Updates the database instance with a new log line created from the arguments and returns the result of the operation
    ///
    /// As long as all arguments implemnt `Into<String>`, the database instance will
    /// Starts by writing to the write-ahead log (WAL) and then updates the internal storage
    /// If the WAL size has exceeded 4 KB (page size), then a checkpoint is created using the `create_checkpoint` function
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use fivewsdb::db::*;
    /// let mut db = FiveWsDB::new("./db_path");
    /// db.update("User123", "Access Denied", "2020-12-30T09:28:57Z", "Login page", "Wrong username or password").expect("Failed to update the database");
    /// ```
    pub fn update<T: Into<String>>(
        &mut self,
        who: T,
        what: T,
        when: T,
        r#where: T,
        why: T,
    ) -> DbResult<()> {
        let entry = FiveWsEntry::new(who, what, when, r#where, why);
        let current_wal_size = self.wal.write(&entry).map_err(|_| DbError::WriteError)?;
        self.storage.push(entry);
        if current_wal_size >= PAGE_SIZE {
            self.create_checkpoint()
                .map_err(|_| DbError::CheckpointError)?;
        }

        Ok(())
    }

    // TODO: Must test this function more as there are various things that can go wrong
    //
    pub fn create_checkpoint(&mut self) -> std::io::Result<()> {
        let entries = self.read("*");
        let f = fs::File::create("tmp")?;
        let mut writer = BufWriter::new(f);
        for e in entries {
            let checkpoint_entry = format!("{}\n", e.to_string());
            writer.write(checkpoint_entry.as_bytes())?;
        }
        let new_checkpoint_file = format!("{}/checkpoint{}.lidb", self.path, self.checkpoint + 1);
        fs::rename("tmp", new_checkpoint_file)?;
        fs::remove_file(format!("{}/checkpoint{}.lidb", self.path, self.checkpoint))?;

        let log_file_location = format!("{}/log{}.lidb", self.path, self.checkpoint + 1);
        fs::File::create(&log_file_location)?;
        fs::remove_file(format!("{}/log{}.lidb", self.path, self.checkpoint))?;

        let mut meta_file = fs::File::create("tmp")?;
        let new_checkpoint = self.checkpoint + 1;
        meta_file.write(&new_checkpoint.to_string().as_bytes())?;
        fs::rename("tmp", format!("{}/meta", self.path))?;

        self.checkpoint += 1;
        // If we don't reintialize the  write-ahead-logger it will contine to insert into the old log file
        // And the file size of the old log file is read and eventually it gets so big that for each write into the
        // database, a new checkpoint is created
        self.wal = WAL::new(log_file_location);

        Ok(())
    }

    pub fn read(&self, pattern: &str) -> Vec<FiveWsEntry> {
        self.storage
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

// fn init_from_checkpoint(checkpoint_location: String) -> DbResult<Vec<FiveWsEntry>> {
fn init_from_checkpoint(checkpoint_file_path: String) -> std::io::Result<Vec<FiveWsEntry>> {
    let mut storage = Vec::new();
    let checkpoint_file = std::fs::File::open(checkpoint_file_path)?;
    let reader = BufReader::new(checkpoint_file);

    // Initalizing from checkpoint file
    for l in reader.lines() {
        let l = l?;
        let vec: Vec<&str> = l.split("|").collect();
        storage.push(FiveWsEntry::from(vec));
    }

    Ok(storage)
}
