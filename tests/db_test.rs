use std::sync::{Arc, RwLock};
use std::thread;

use fivewsdb::db::*;

const TESTS_DIR_PATH: &str = "./tests/lidb";

pub fn teardown(path: &str) {
    println!("Cleaning files. Path: '{}'", path);
    // We want the tests to quit if we fail to tear everything down
    // Otherwise the rest of the integration tests might fail
    std::fs::remove_dir_all(path).expect("Failed to teardown directory");
}

#[test]
fn test_database_update() {
    let mut db = FiveWsDB::new(TESTS_DIR_PATH);

    db.update("Ingi", "Job start", "2020-20-12", "", "").unwrap();
    db.update("IT guy", "Job start", "2020-20-12", "", "").unwrap();
    db.update("Office guy", "Job start", "2020-20-12", "", "").unwrap();

    let entries = db.read("Ingi");

    assert_eq!(entries.len(), 1);

    let entry = entries[0].clone();
    assert_eq!(entry.to_string(), "Ingi|Job start|2020-20-12||");

    teardown(TESTS_DIR_PATH);
}

#[test]
fn test_multiple_readers() {
    let mut db = FiveWsDB::new(TESTS_DIR_PATH);

    db.update("ingi", "", "", "", "").unwrap();

    let thread_safe_db = Arc::new(RwLock::new(db));

    let handlers: Vec<_> = (0..10)
        .map(|_| {
            let db_clone = thread_safe_db.clone();
            thread::spawn(move || {
                let r = db_clone.read().unwrap();
                let entries = r.read("ingi");
                assert_eq!(entries.len(), 1);
            })
        })
        .collect();

    for h in handlers {
        h.join().unwrap();
    }

    teardown(TESTS_DIR_PATH);
}

#[test]
fn test_multiple_writers() {
    let db = FiveWsDB::new(TESTS_DIR_PATH);

    let thread_safe_db = Arc::new(RwLock::new(db));

    let handlers: Vec<_> = (0..10)
        .map(|_| {
            let db_clone = thread_safe_db.clone();
            thread::spawn(move || {
                let mut w = db_clone.write().unwrap();
                w.update("ingi", "", "", "", "").unwrap();
            })
        })
        .collect();

    for h in handlers {
        h.join().unwrap();
    }

    assert_eq!(thread_safe_db.read().unwrap().read("ingi").len(), 10);

    teardown(TESTS_DIR_PATH);
}

#[test]
fn test_large_number_of_writes() {
    let db = FiveWsDB::new(TESTS_DIR_PATH);

    let number_of_threads = 10;
    let number_of_writes = 1000;

    let thread_safe_db = Arc::new(RwLock::new(db));

    let handlers: Vec<_> = (0..number_of_threads)
        .map(|thread_num| {
            let db_clone = thread_safe_db.clone();
            thread::spawn(move || {
                let mut w = db_clone.write().unwrap();
                for i in 0..number_of_writes {
                    w.update(format!("{}-{}", thread_num, i).as_str(), "", "", "", "")
                        .unwrap();
                }
            })
        })
        .collect();

    for h in handlers {
        h.join().unwrap();
    }

    assert_eq!(
        thread_safe_db.read().unwrap().read("*").len(),
        number_of_threads * number_of_writes
    );

    teardown(TESTS_DIR_PATH);
}

#[test]
fn test_checkpoint_manual_creation() {
    let mut db = FiveWsDB::new(TESTS_DIR_PATH);

    db.update("ingi", "", "", "", "").unwrap();

    db.create_checkpoint().unwrap();

    let new_checkpoint_exists = std::path::Path::new(format!("{}/checkpoint1.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(new_checkpoint_exists, true);
    let new_log_exists = std::path::Path::new(format!("{}/log1.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(new_log_exists, true);

    let old_checkpoint_exists = std::path::Path::new(format!("{}/checkpoint0.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(old_checkpoint_exists, false);
    let old_log_exists = std::path::Path::new(format!("{}/log0.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(old_log_exists, false);

    teardown(TESTS_DIR_PATH);
}

#[test]
fn test_checkpoint_automatic_creation() {
    let mut db = FiveWsDB::new(TESTS_DIR_PATH);

    
    for _ in 0..500 {
        db.update("", "", "", "", "").unwrap();
    }

    let new_checkpoint_exists = std::path::Path::new(format!("{}/checkpoint1.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(new_checkpoint_exists, true);
    let new_log_exists = std::path::Path::new(format!("{}/log1.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(new_log_exists, true);

    let old_checkpoint_exists = std::path::Path::new(format!("{}/checkpoint0.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(old_checkpoint_exists, false);
    let old_log_exists = std::path::Path::new(format!("{}/log0.lidb", TESTS_DIR_PATH).as_str()).exists();
    assert_eq!(old_log_exists, false);

    teardown(TESTS_DIR_PATH);
}
