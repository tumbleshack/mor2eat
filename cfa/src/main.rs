use std::env;
use std::process;

use cfa::Arg;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = Arg::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("In file {}", arg.dir);

    if let Err(e) = cfa::run(arg) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}