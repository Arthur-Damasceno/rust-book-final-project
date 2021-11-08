use std::{io::Error, net::TcpListener};
use thread_pool::ThreadPool;

mod http;

use http::handle_connection;

pub fn run(host: &str, port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(&mut stream);
        })
    }

    Ok(())
}
