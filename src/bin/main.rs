use simple_http::Cmd;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = Cmd::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    Cmd::run(cmd);
}

