/*use std::fs::File;


//static mut initalized: bool = false;

pub struct Logger {
    pub stream: File,
}

impl Logger {
    pub fn initalize() -> Logger {
        match File::open("") {
            Ok(file) => {},
            Err(_) => {},
        }
        Logger {
            stream: file,
        }
    }
}


impl Drop for Logger {
    fn drop(&mut self) {
    
        println!("Exiting.");
    }
}

fn find_index() -> u32 {
    File::
}*/