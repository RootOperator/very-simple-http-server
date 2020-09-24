#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

use simple_http::Server;
use std::env;
use std::process;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 && args.len() != 2 {
        eprintln!("not enough arguments provided");
        process::exit(1);
    }
    
    let host: String;
    let port: i16;
    let query: String = args[1].clone();
    let path = Path::new(&query);

    let (host, port) = if args.len() == 2 {
        (String::from("127.0.0.1"), 8000)
    } else {
        (args[2].clone(), args[3].clone().parse::<i16>().unwrap())
    };
    
    let mut server = Server::connect(&host, port);


    if path.exists() {
        if path.is_file() {
            server.add("/", &query);
            server.run();
        } else {         
            let dir_items = path.read_dir().unwrap();

            for i in dir_items {
                println!("{:?}", i.unwrap().path());
            }
        }
    }

}

