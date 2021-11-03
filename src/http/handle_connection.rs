use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use super::{response::Response, status::Status};

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        Response::html("public/hello.html").unwrap()
    } else if buffer.starts_with(sleep) {
        let mut response = Response::html("public/404.html").unwrap();
        response.status(Status::NotFound);

        thread::sleep(Duration::from_secs(5));

        response
    } else {
        let mut response = Response::html("public/404.html").unwrap();
        response.status(Status::NotFound);

        response
    };

    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
