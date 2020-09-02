use simple_http::Server;

fn test() {

}

fn main() {
    let mut listener = Server::connect("127.0.0.1", 8000);
    listener.add("/sleep", "index.html", &test);

    listener.run();
}

