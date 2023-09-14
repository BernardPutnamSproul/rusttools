pub mod http;
use std::{
    fs,
    net::{
        TcpListener,
        TcpStream
    }, 
    io::{
        BufReader, 
        prelude::*
    },
};

pub fn run(address: &str) {
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to create server.");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    Box
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    
    let http_request: Vec<String> = buffer_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    /*
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();*/

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("html/response.html").unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
