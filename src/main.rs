use crate::server::Server;
use simplelog::*;
use std::fs::File;

mod handler;
mod log;
mod pool;
mod request;
mod response;
mod router;
mod server;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("log/web.log").unwrap(),
        ),
    ])
    .unwrap();
    let server = Server::new("localhost:8888");
    server.run();
}
