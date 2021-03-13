use SimpleHTTP::Server;
use std::env;
use std::process;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 && args.len() != 2 && args.len() != 3 {
        eprintln!("not enough arguments provided");
        process::exit(1);
    }

    let query: String = args[1].clone();

    let (host, port) = if args.len() == 2 {
        ("127.0.0.1".to_string(), 8000)
    } else if args.len() == 3 {
        ("127.0.0.1".to_string(), args[2].clone().parse::<i16>().unwrap())
    } else {
        (args[2].clone(), args[3].clone().parse::<i16>().unwrap())
    };

    let mut server = Server::connect(&host, port);
    let path = Path::new(&query);

    if path.exists() {
        server.logic(&path, &query);
        server.run();
    } else {
        eprintln!("File or directory not found");
    }
}
