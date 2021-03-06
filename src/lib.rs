use std::io::prelude::*;
use std::net::{TcpListener, IpAddr, TcpStream, SocketAddr};
use std::fs;
use std::path::Path;
use std::collections::HashMap;


pub struct Server {
    listener: TcpListener,
    pub routes: HashMap<String, String>
}


impl Server {
    pub fn connect(host: &str, port: i16) -> Server {
        let bind = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&bind).unwrap();
        println!("Now listening on {}", &bind);
        let routes = HashMap::new();
        return Server { listener, routes };
    }

    pub fn add(&mut self, route: &str, filename: &str) {
        self.routes.insert(String::from(route), String::from(filename));
    }

    pub fn run(self) {
        let mut contents: String = String::new();
        let mut status_line: &str = "HTTP/1.1 200 OK\r\n\r\n";
        for stream in self.listener.incoming() {
            let mut stream: TcpStream = stream.unwrap();
            let peer_addr: SocketAddr = stream.peer_addr().unwrap();
            let peer_ip: IpAddr = peer_addr.ip();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let s = format!("{}", String::from_utf8_lossy(&buffer[..]));
            let o = s.trim_end_matches("\u{0}");
            println!("{}", o);

            for route in &self.routes {
                let search = format!("GET {} HTTP/1.1", route.0);

                if buffer.starts_with(search.as_bytes()) {
                    let path = Path::new(route.1);

                    if path.is_file() {
                        contents = fs::read_to_string(route.1).unwrap();
                        status_line = "HTTP/1.1 200 OK\r\n\r\n";
                    } else {
                        contents = create_dir_html(route.1.to_string(), &self.routes);
                    }
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

    pub fn logic(&mut self, path: &Path, query: &str) {
        &self.add("/", &query);

        if path.is_dir() {
            let dir_items = path.read_dir().unwrap();

            for i in dir_items {
                let item = format!("{}", i.as_ref().unwrap().path().to_str().unwrap());
                let mut format = String::from(&item);
                let new_path = Path::new(&item);

                let range = if query.ends_with("/") {
                    format.find(&query).unwrap() + &query.len() - 1
                } else {
                    format.find(&query).unwrap() + &query.len()
                };

                format.replace_range(..range, "");

                if new_path.is_dir() {
                    &self.logic(&new_path, &query);
                }

                &self.add(&format, &item);
            }
        }
    }
}

pub fn create_dir_html(dir: String, routes: &HashMap<String, String>) -> String {
    let mut html = String::from("<!DOCTYPE html><html><body>");
    let url = if dir.contains("/") {
        let split: Vec<&str> = dir.split("/").collect();
        let index = split.len() - 2;
        if index == 0 {
            "/".to_string()
        } else {
            let mut previous_dir: String = String::from("");

            for n in 1..=index {
                let dir_name = format!("/{}", split[n]);
                previous_dir.push_str(&dir_name);
            }
            previous_dir
        }
    } else { "/".to_string() };

    let parent_dir = format!("<a href='{}'>../</a><br>", url);
    html.push_str(&parent_dir);

    for route in routes {
        let parent = Path::new(route.1).parent().unwrap();
        if route.1.starts_with(&dir) && parent == Path::new(&dir) {
            let mut link: String = String::from(route.0);
            if Path::new(route.1).is_dir() { link.push_str("/") }
            link = format!("<a href='{url}'>{link}</a><br>", url=route.0, link=link);
            html.push_str(&link);
        }
    }

    html.push_str("</body><style>body {
                    font-family: Courier new;
                    display: inline-block;
                    position: absolute;
                    background-color: #0f1419;
                    left: 40%; 
                    top: 12%;}
                    a, a:hover, a:visited, a:active {
                    color: #b9b9ba;
                    text-decoration: none;
                    }</style></body></html>");

    return html;
}

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
