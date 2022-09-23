use super::router::Router;
use crate::pool::ThreadPool;
use crate::request::HttpRequest;
use log::info;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        info!("Running on {}", self.socket_addr);
        let thread_pool = ThreadPool::new(10);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            info!("Connection established");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();

            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            thread_pool.execute(move || {
                Router::route(req, &mut stream);
            });
        }
    }
}
