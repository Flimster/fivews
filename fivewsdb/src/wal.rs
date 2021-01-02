// WAL - Write ahead logging for the database

use std::fs;
use std::io::{self, prelude::*, BufReader};

use crate::entry::LogEntry;

// Write ahead logger
pub struct WAL {
    f: fs::File,
}

impl WAL {
    pub fn new(log_location: String) -> WAL {
        let f = fs::OpenOptions::new()
            .append(true)
            .truncate(false)
            .read(true)
            .open(log_location)
            .expect("Failed to create WAL");

        WAL { f }
    }

    // Returns the size of the write-ahead file
    pub fn write(&mut self, entry: &LogEntry) -> io::Result<u64> {
        self.f.write_all(format!("{}\n", entry).as_bytes())?;
        let length = self.f.metadata()?.len();
        Ok(length)
    }

    pub fn get_logs(&self) -> Vec<LogEntry> {
        let reader = BufReader::new(&self.f);

        reader
            .lines()
            .map(|l| l.unwrap_or_default())
            .filter(|l| !l.is_empty())
            .map(|l| LogEntry::from(l.split("|").collect()))
            .collect()
    }
}
