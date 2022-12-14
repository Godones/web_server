use super::router::Router;
use crate::pool::ThreadPool;
use crate::request::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
use std::string::String;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self, thread_number: usize) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {} threads: {}", self.socket_addr, thread_number);
        let thread_pool = ThreadPool::new(thread_number);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            thread_pool.execute(move || {
                Router::route(req, &mut stream);
            });
        }
    }
}
