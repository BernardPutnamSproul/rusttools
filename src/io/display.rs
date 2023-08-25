use std::io::{self, Write};


pub const RST: &str = "\u{1b}[0m";
pub const BLK: &str = "\u{1b}[01;30m";
pub const RED: &str = "\u{1b}[01;31m";
pub const GRN: &str = "\u{1b}[01;32m";
pub const YLW: &str = "\u{1b}[01;33m";
pub const BLU: &str = "\u{1b}[01;34m";
pub const MAG: &str = "\u{1b}[01;35m";
pub const CYN: &str = "\u{1b}[01;36m";
pub const LGR: &str = "\u{1b}[01;37m";
pub const WHT: &str = "\u{1b}[01;38m";


pub fn progress_bar(limit: u64, progress: u64) {
    let percentage: f32 = (progress as f32 / limit as f32) * 100.0;
    let mut output: String = String::from("\r\u{1b}[01;30mProgress: [");
    let length = limit.to_string().len();
    
    output.push_str(GRN);
    
    if percentage < 1.0 {
        output.push_str("\u{1b}[01;31m>---------------------------------------------------------------------------------------------------");
    } else if percentage > 99.9 {
        output.push_str("====================================================================================================");
    } else {

        for _i in 0..(percentage - 1.0) as u64 {
            output.push('=');
        }    

        output.push_str(">\u{1b}[01;31m");
        
        for _i in percentage as u64..100 {
            output.push('-');
        }    
    }

    output.push_str(&format!("{BLK}] {progress:length$}/{limit} ({percentage:6.2}%)"));
    print!("{output}{RST}");

    io::stdout().flush().expect("Error while flushing stdout.");
}