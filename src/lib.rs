use std::{io::Error, net::TcpListener};

pub fn run(host: &str, port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }

    Ok(())
}
