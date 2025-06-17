use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //* Wraps around stream, intead of reading data from the stream. It reads larger chunks at once.
    let buf_reader = BufReader::new(&stream);
    
    //* HTTP Request: Is a vector of strings that reads and parses each line.
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //* Response Header
    //* HTML VERSION - STATUS CODE - PHRASE - CRLF */ 
    let statusline = "HTTP/1.1 200 OK";
    
    //* Reading File Contents (sample.html) 
    let contents = fs::read_to_string("./sample.html").unwrap();
    let length = contents.len();
    
    //* Formatted Response
    let response = format!("{statusline}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //* Returning a Response */
    stream.write_all(response.as_bytes()).unwrap();
}
