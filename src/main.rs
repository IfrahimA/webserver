use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    //* TCP LISTENER: Listens for incoming connections */
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //* ITERATE: ASYNC */
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //* HANDLE CONNECTIONS */
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //* Wraps around stream, manages the stream by applying a buffer.
    let buf_reader = BufReader::new(&stream);

    //* HTTP Request -> First Line */
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //* Request Header */
    //* METHOD - ROUTE - HTML VERSION - CRLF */
    let (statusline, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/sample.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "./src/404.html")
    };

    //* Response Header */
    //* HTML VERSION - STATUS CODE - PHRASE - CRLF */
    let contents: String = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{statusline}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
