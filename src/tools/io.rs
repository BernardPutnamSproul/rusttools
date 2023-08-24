pub mod display;
use std::{io, str::FromStr};


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