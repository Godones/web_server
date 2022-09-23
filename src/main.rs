use crate::server::Server;

mod handler;
mod log;
mod pool;
mod request;
mod response;
mod router;
mod server;

fn main() {
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    let server = Server::new("localhost:8888");
    server.run();
}
