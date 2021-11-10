mod http;

use {
    async_std::{io::Error, net::TcpListener, task::spawn},
    futures::stream::StreamExt,
    http::handle_connection,
};

pub async fn run(host: &str, port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;

    Ok(())
}
