use std::{io::Error, net::TcpListener};

mod handle_connection;

use handle_connection::handle_connection;

pub fn run(host: &str, port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())
}
