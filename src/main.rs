#[async_std::main]
async fn main() {
    let host = "127.0.0.1";
    let port = 8000;

    web_server::run(host, port).await.unwrap();
}
