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

#[cfg(test)]
mod tests {
    use {
        super::*,
        async_std::fs,
        futures::task::{Context, Poll},
        std::{cmp, io::Result, marker::Unpin, pin::Pin},
    };

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: String,
        is_flush_called: bool,
    }

    impl MockTcpStream {
        fn new(read_data: &str) -> Self {
            Self {
                read_data: Vec::from(read_data.as_bytes()),
                write_data: String::new(),
                is_flush_called: false,
            }
        }
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize>> {
            let size = cmp::min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize>> {
            self.write_data = String::from_utf8_lossy(buf).to_string();
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
            self.is_flush_called = true;
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockTcpStream {}

    #[async_std::test]
    async fn should_handle_connection() {
        let mut stream = MockTcpStream::new("GET / HTTP/1.1\r\n");

        handle_connection(&mut stream).await;
        let body = fs::read_to_string("public/hello.html").await.unwrap();

        assert!(stream.is_flush_called);
        assert!(stream.write_data.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(stream.write_data.ends_with(&body));
    }
}
