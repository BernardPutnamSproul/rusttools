use std::thread::{self, JoinHandle};


pub const RST: &str = "[0m";
pub const BLK: &str = "[0m[01;30m";
pub const RED: &str = "[0m[01;31m";
pub const GRN: &str = "[0m[01;32m";
pub const YLW: &str = "[0m[01;33m";
pub const BLU: &str = "[0m[01;34m";
pub const MAG: &str = "[0m[01;35m";
pub const CYN: &str = "[0m[01;36m";
pub const LGR: &str = "[0m[01;37m";
pub const WHT: &str = "[0m[01;38m";


pub fn progress_bar(limit: u64, progress: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        print!("\rProgress: [{GRN}");
    
        for _i in 0..((progress as f32 / limit as f32) * 100.0) as u64 {
            print!("=");
        }
        if limit != progress {
            print!(">{RED}");
        }
        for _i in ((progress as f32 / limit as f32) * 100.0) as u64..100 {
            print!("-");
        }
    
        print!("{RST}] {progress}/{limit} (0.0%)");
    })
}