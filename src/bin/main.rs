#![allow(dead_code, unused_imports)]

use simple_http::Server;
use std::env;
use std::process;

fn main() {
    let mut server = Server::connect("127.0.0.1", 8000);
    server.add("/test", "index.html");
    server.run();
}

