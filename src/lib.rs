use std::io::prelude::*;
use std::net::TcpListener;
use std::fs;
use std::collections::HashMap;


pub struct Server<'key, 'value> {
   listener: TcpListener,
   pub routes: HashMap<&'key str, (&'key str, &'value dyn Fn())>
}

impl<'key, 'value> Server<'key, 'value> {
    pub fn connect(host: &str, port: i16) -> Server {
        let bind = format!("{}:{}", host, port);
        let listener = TcpListener::bind(bind).unwrap();
        let routes = HashMap::new();
        return Server { listener, routes }
    }

    pub fn add(&mut self, route: &'key str, filename: &'key str, function: &'value dyn Fn()) {
        self.routes.insert(route, (filename, function));
    }

    pub fn run(&mut self) {
        let mut contents: String = String::from("");
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0;1024];
            stream.read(&mut buffer).unwrap();

            for route in &self.routes {
                let search = format!("GET {} HTTP/1.1", route.0);
                println!("{}", search);

                if buffer.starts_with(search.as_bytes()) {
                    contents = fs::read_to_string("index.html").unwrap();
                    println!("YEP");
                    break;
                } else {
                    contents = String::from("html");
                }
            }

            let status_line = "HTTP/1.1 200 OK\r\n\r\n";
            //let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", status_line, contents);
            

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

