use std::{error::Error, time::{self, Instant}};
use tools::{serialization, io::display::{self, progress_bar}};

pub mod tools;


fn main() {

    println!("{}", usize::MAX);
    println!("{}", u64::MAX);
    return;
    let limit: u32 = 11;
    for iteration in 3..limit + 1 {
        progress_bar(limit as u64, iteration as u64);

        let mut vector: Vec<u64> = Vec::new();
        //println!("Iteration: {iteration}. Length: {}", (10 as u64).pow(iteration));

        for i in 0..(10 * iteration) {
            vector.push(i.into());
        }

        //let begining: time::Instant = Instant::now();
        //print!("Serializing...");
        
        serialization::serializeu64("test/vector.bin", &vector).unwrap_or_else(|err: Box<dyn Error>| {
            panic!("Error while serializing vector: {err}");
        });
        
        //println!("     Completed. Time: {:#?}", Instant::now() - begining);
    
    
        //let begining: time::Instant = Instant::now();
        //print!("Deserializing...");
        
        let _vector: Vec<u64> = match serialization::deserializeu64("test/vector.bin") {
            Ok(output) => output,
            Err(err) => {
                panic!("Error during deserialization {err}");
            },
        };
        
        //println!("   Completed. Time: {:#?}", Instant::now() - begining);
    }
}
