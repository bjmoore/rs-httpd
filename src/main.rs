use server::HttpServer;

mod connection;
mod request;
mod response;
mod server;

const LOCAL_SOCK: &'static str = "127.0.0.1:8001";

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(&LOCAL_SOCK)?;
    server.run()
}
