use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;

fn init_files(dir_path: &str, checkpoint: usize) {
    // We don't care whether these operations succeed or not since they lead to the same result
    // Ok(file) => File did not exist and this operation created it
    // Err(e) => File exists and there is no need to do anything about it
    // In the end the result is the same, a checkpoint file and a log file

    let checkpoint_path = format!("{}/checkpoint{}.lidb", dir_path, checkpoint);
    let log_path = format!("{}/log{}.lidb", dir_path, checkpoint);

    let _ = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(checkpoint_path);

    let _ = fs::OpenOptions::new().write(true).create_new(true).open(log_path);
}

pub fn init_lidb(dir_path: &str) -> usize {
    let meta_directory = format!("{}/meta", dir_path);
    let mut buffer = String::new();

    match fs::create_dir(&dir_path) {
        Ok(()) => {
            // Create the meta file and initilize with 0
            let _ = fs::File::create(meta_directory)
                .expect("Unable to create meta file")
                .write("0".as_bytes())
                .expect("Unable to intialize checkpoint");
            init_files(dir_path, 0);
            0
        }
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
            // Directory already exists
            // We assume that the folloowing files also exist:
            // The log file
            // The checkpoint file
            // The meta file
            let checkpoint = fs::File::open(meta_directory)
                .map(|mut f| {
                    f.read_to_string(&mut buffer)
                        .ok()
                        .map(|_| buffer.parse::<usize>().unwrap_or(0))
                        .unwrap_or(0)
                })
                .unwrap_or(0);
            init_files(dir_path, checkpoint);
            checkpoint
        }
        Err(e) => panic!("Unable to initialize lowiq database: {}", e),
    }
}
