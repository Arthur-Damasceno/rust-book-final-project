use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::response::Response;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = Response::html("public/hello.html").unwrap();

    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
