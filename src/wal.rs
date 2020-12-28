// WAL - Write ahead logging for the database

use std::fs;
use std::io::{self, prelude::*, BufReader};

use crate::entry::FiveWsEntry;

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
    pub fn write(&mut self, entry: &FiveWsEntry) -> io::Result<u64> {
        self.f.write_all(format!("WRITE{}\n", entry).as_bytes())?;
        let length = self.f.metadata()?.len();
        Ok(length)
    }

    pub fn get_logs(&self) -> Vec<(String, FiveWsEntry)> {
        let reader = BufReader::new(&self.f);

        // TODO: Handle if logs are not in the correct format
        reader
            .lines()
            .map(|l| l.unwrap_or_default())
            .filter(|l| !l.is_empty())
            .map(|l| {
                let operation = l[0..5].to_string();
                let entry = l[5..].to_string();
                (operation, FiveWsEntry::from(entry.split("|").collect()))
            })
            .collect()
    }

    pub fn get_wal_size(&mut self) -> std::io::Result<u64> {
        let metadata = self.f.metadata()?;
        Ok(metadata.len())
    }

    pub fn clear_logs(&mut self) -> std::io::Result<()> {
        self.f.set_len(0)?;
        Ok(())
    }
}
