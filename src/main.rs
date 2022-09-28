mod actions;
mod base;

use std::env;
use actions::{Program};

fn main(){

    let mut program = Program::new();
    program.init_table();

    let args: Vec<String> = env::args().collect();
    let vars;
    
    //TODO: I need proper error checking here
    // if let Some(&args[2..]) = &args[2..].join(" ");
    if args.len() > 2 {
        vars = args[2..].join(" ");
    } else {
        vars = String::from("").to_owned();
    }

    // String into slice:
    match &args[1][..]{
        "-c" => program.create(&vars),
        "-r" => program.read(&vars),
        "-u" => program.update(&vars),
        "-d" => program.delete(&vars),
        "-a" => program.all(&vars),
        "-h" => program.help(),
        _ => program.unknown(),
    }
}
