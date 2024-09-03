use server::HttpServer;

mod connection;
mod request;
mod response;
mod server;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // load configuration

    let server = HttpServer::new(&LOCAL_SOCK)?;
    server.run().await
}
