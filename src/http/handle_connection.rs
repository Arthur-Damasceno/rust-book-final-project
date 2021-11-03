use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::response::Response;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        Response::html("public/hello.html")
    } else {
        Response::html("public/404.html")
    }
    .unwrap();

    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
