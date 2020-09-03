use std::io::prelude::*;
use std::path::Path;
use std::net::TcpListener;
use std::fs;
use std::collections::HashMap;

// TODO 
// make is_file actually check if it is a file and not just if the path exists
// create tests
// write simple file server if a path is suppliad as an argument instead of a file


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

    pub fn fn_add(&mut self, route: &'key str, filename: &'key str, function: &'value dyn Fn()) {
        self.routes.insert(route, (filename, function));
    }
    

    pub fn add(&mut self, route: &'key str, filename: &'key str) {
        self.routes.insert(route, (filename, &nothing));
    }

    pub fn run(&mut self) {
        let mut contents: String = String::from("");
        let mut status_line: &str = "HTTP/1.1 418 I'm a teapot";
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0;1024];
            stream.read(&mut buffer).unwrap();

            for route in &self.routes {
                let search = format!("GET {} HTTP/1.1", route.0);

                if buffer.starts_with(search.as_bytes()) {
                    contents = fs::read_to_string((route.1).0).unwrap();
                    status_line = "HTTP/1.1 200 OK\r\n\r\n";
                    (route.1).1();
                    break;
                } else {
                    contents = load_404();
                    status_line = "HTTP/1.1 404 Not Found\r\n\r\n";
                }
            }

            let response = format!("{}{}", status_line, contents);
            

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

pub struct Cmd {
    pub host: String,
    pub port: i16,
    pub path: String 
}


impl Cmd {
    pub fn new(args: &[String]) -> Result<Cmd, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough arguments");
        }
    
        let host = args[1].clone();
        let port = args[2].clone().parse::<i16>().unwrap();
        //let port = Port.parse<i16>.unwrap();
        let path = args[3].clone();
        
        return Ok(Cmd { host, port, path })
    }

    pub fn run(cmd: Cmd) {
        let mut server = Server::connect(&cmd.host, cmd.port);
        if is_file(&cmd.path) {
            server.add("/", &cmd.path);
            server.run();
        } else {
            // server.runFileServer
        }
    }
}

fn is_file(query: &str) -> bool {
    return Path::new(&query).exists();
}

fn nothing() {}
fn load_404() -> String {
    let html = "\
        <!DOCTYPE html>
            <html>
                <body>
                    <h1>404</h1>
                    <h3>Page not found</h3>
                </body>
            <style>
            body {
                font-family: Courier new;
                max-width: 500px;
                padding-top: 20%;
                text-align: center;
                margin: auto;
            }
            </style>
            </html>
        ";
    return String::from(html);
}

