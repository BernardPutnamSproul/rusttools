use std::{fs::{File, self, ReadDir}, io::{Write, Error}};

use chrono::Local;


const INFO: &str = " [Info]: ";
const WARN: &str = " [Warn]: ";
const DIRECTORY: &str = "DataLogs/";

pub struct Logger {
    pub stream: File,
}

impl Logger {
    pub fn initalize() -> Logger {
        let mut output: Logger = Logger {
            stream: create_log_file().unwrap(),
        };
        output.log_info("Logger Started.");

        output
    }

    pub fn log_time(&mut self) {
        self.stream.write_all(
            Local::now()
            .format("[%F %H:%M:%S%.3f]")
            .to_string()
            .as_bytes()
        ).expect("Error logging time.");
    }

    pub fn log_info(&mut self, message: &str) {
        self.log_time();
        self.stream.write_all(
            format!("{INFO}{message}\n")
            .as_bytes()
        ).expect("Error logging info.");
    }

    pub fn log_warn(&mut self, message: &str) {
        self.log_time();
        self.stream.write_all(
            format!("{WARN}{message}\n")
            .as_bytes()
        ).expect("Error logging info.");
    }
}


impl Drop for Logger {
    fn drop(&mut self) {
        println!("Exiting.");
    }
}

fn create_log_file() -> Result<File, Error> {
    let directory: ReadDir = loop {
        match fs::read_dir(DIRECTORY) {
            Err(_) => {
                fs::create_dir(DIRECTORY)?;
                // Adds "DataLogs/" to the .gitignore file.
                File::options()
                    .append(true)
                    .open(".gitignore")
                    ?.write_all(DIRECTORY.as_bytes())?;
                continue;
            },
            Ok(dir) => break dir,
        }
    };

    let file_number: usize = directory.count();
    
    File::create(format!("{DIRECTORY}log{file_number}.log"))
}