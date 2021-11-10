use {
    super::{response::Response, status::Status},
    async_std::{
        io::{Read, Write},
        prelude::*,
        task,
    },
    std::{marker::Unpin, time::Duration},
};

pub async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        Response::html("public/hello.html").await.unwrap()
    } else if buffer.starts_with(sleep) {
        let mut response = Response::html("public/404.html").await.unwrap();
        response.status(Status::NotFound);
        task::sleep(Duration::from_secs(5)).await;
        response
    } else {
        let mut response = Response::html("public/404.html").await.unwrap();
        response.status(Status::NotFound);
        response
    };

    stream.write(response.to_string().as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
