pub mod display;
use std::{io, str::FromStr};


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

pub fn get<T: FromStr>() -> Option<T> {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while getting input.");

    match input.trim().parse::<T>() {
        Ok(output) => Some(output),
        Err(_) => None,
    }
}


/// make this good.
pub fn internal_trim(mut input: String) -> String {
    // String preprocessing.
    while input.contains("  ") {
        input = input.replace("  ", " ");
    }
    input.trim()
        .to_string()
        .replace('\t', " ")
        .replace('\n', " ")
}
/*
pub fn get<T: FromStr>(default: T) -> T {
    let mut input: String = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("Error while getting input.");

match input.trim().parse::<T>() {
    Ok(output) => output,
    Err(_) => default,
}
}
*/