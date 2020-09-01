use std::io::prelude::*;
//use std::net::TcpListener;
use std::net::TcpStream;
//use std::fs;
//use std::io::Result;
//use std::collections::HashMap;

use simple_http::Server;

fn test() {

}

fn main() {
    //let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut listener = Server::connect("127.0.0.1", 8000);
    //let s = listener.listener.unwrap();
    listener.add("/sleep", "index.html", &test);
    //for stream in s.incoming() {
        //let stream = stream.unwrap();

        //handle_connection(stream)
    //}
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//struct Server<'key, 'value> {
    //host: &'key str,
    //port: i16,
    //listener: Result<TcpListener>,
    //routes: HashMap<&'key str, (&'key str, &'value dyn Fn())>
//}

//impl<'key, 'value> Server<'key, 'value> {
    //fn connect(host: &str, port: i16) -> Server {
        //let bind = format!("{}:{}", host, port);
        //let listener =  TcpListener::bind(bind);
        //let routes = HashMap::new();
        //return Server { host, port, listener, routes }
    //}

    //fn add(&mut self, route: &'key str, filename: &'key str, function: &'value dyn Fn()) {
        //self.routes.insert(route, (filename, function));
    //}
//}

