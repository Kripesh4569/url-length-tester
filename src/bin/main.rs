extern crate regex;

// use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use regex::Regex;
use url_length_tester::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    
    let re = Regex::new(r"GET\s(?P<URI>[^\s]+)\sHTTP/1.1").unwrap();
    let caps = re.captures(&request).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = &caps["URI"];
    // let contents = request;

    let response = format!{
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
