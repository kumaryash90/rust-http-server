use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );

    let contents = fs::read_to_string("index.html").unwrap();

    // response format:
    // HTTP-version Status-code Reason-phrase CRLF
    // headers
    // message body
    //
    // e.g. HTTP/1.1 200 OK\r\n\r\n
    
    let response_short = "HTTP/1.1 200 OK\r\n\r\n";
    let response_full = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response_full.as_bytes()).unwrap();
    stream.flush().unwrap();
}