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

    let (host, port) = if args.len() == 2 {
        (String::from("127.0.0.1"), 8000)
    } else {
        (args[2].clone(), args[3].clone().parse::<i16>().unwrap())
    };
    
    let mut server = Server::connect(&host, port);
    let path = Path::new(&query);
    

    if path.exists() {
        logic(&mut server, &path, &query);
        println!("{:#?}", server.routes);
        server.run();
    } else {
        eprintln!("File or directory not found");
    }
}

fn logic(mut server: &mut Server, path: &Path, query: &str) {
    if path.is_file() {
        server.add("/", &query);
    } else {
        let dir_items = path.read_dir().unwrap();

        for i in dir_items {
            let item = format!("{}", i.as_ref().unwrap().path().to_str().unwrap());
            let format = format!("/{}", &item);
            let new_path = Path::new(&item);

            if new_path.is_dir() {
                logic(&mut server, &new_path, &query);
            } else {
                server.add(&format, &item);
            }

        }
    }
}

