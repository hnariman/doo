mod actions;
use std::env;
use std::result::{Result};
use std::fmt::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let vars;
    
    //TODO: I need proper error checking here
    // if args.len() >2 {
    //     vars = args[2..].join(" ");
    // } else {
    //     vars = String::from("");
    // }

    let vars = &args[2..].join(" ").chain_err(|| "problems");
    // converting String into slice:
    match &args[1][..]{
        "-c" => actions::create(vars),
        "-r" => actions::read(vars),
        "-u" => actions::update(vars),
        "-d" => actions::delete(vars),
        "-a" => actions::all(vars),
        "-h" => actions::help(),
        _ => actions::unknown(),
    }
    Ok(())
}
