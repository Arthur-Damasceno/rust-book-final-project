use std::{
    io::{Read, Write},
    thread,
    time::Duration,
};

use super::{response::Response, status::Status};

pub fn handle_connection(stream: &mut (impl Read + Write)) {
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

#[cfg(test)]
mod tests {
    use std::io::{Read, Result, Write};

    use super::*;

    struct MockStream<'a> {
        read_data: &'a str,
        write_data: String,
        is_flush_called: bool,
    }

    impl<'a> MockStream<'a> {
        fn new(read_data: &'a str) -> Self {
            Self {
                read_data,
                write_data: String::new(),
                is_flush_called: false,
            }
        }
    }

    impl<'a> Read for MockStream<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.read_data.as_bytes().read(buf).unwrap();
            Ok(0)
        }
    }

    impl<'a> Write for MockStream<'a> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.write_data.push_str(&String::from_utf8_lossy(buf));
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            self.is_flush_called = true;
            Ok(())
        }
    }

    #[test]
    fn should_handle_request() {
        let mut stream = MockStream::new("GET / HTTP/1.1\r\n\r\n");

        handle_connection(&mut stream);

        assert!(stream.write_data.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(stream.is_flush_called);
    }
}
