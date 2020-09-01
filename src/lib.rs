//use std::io::prelude::*;
use std::net::TcpListener;
//use std::net::TcpStream;
//use std::fs;
use std::io::Result;
use std::collections::HashMap;



pub struct Server<'key, 'value> {
   host: &'key str,
   port: i16,
   listener: Result<TcpListener>,
   routes: HashMap<&'key str, (&'key str, &'value dyn Fn())>
}

impl<'key, 'value> Server<'key, 'value> {
    pub fn connect(host: &str, port: i16) -> Server {
        let bind = format!("{}:{}", host, port);
        let listener = TcpListener::bind(bind);
        let routes = HashMap::new();
        return Server { host, port, listener, routes }
    }

    pub fn add(&mut self, route: &'key str, filename: &'key str, function: &'value dyn Fn()) {
        self.routes.insert(route, (filename, function));
    }
}

