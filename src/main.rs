use std::env;

#[derive(Debug)]
enum Commands {
    Create,
    Read,
    Update,
    Delete,
    Unknown
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // converting String into slice:
    let command = match &args[1][..]{
        "c" => Commands::Create,
        "r" => Commands::Read,
        "u" => Commands::Update,
        "d" => Commands::Delete,
        _ => Commands::Unknown,
    };

    // let result = match command { };

    println!("command is : {:#?}",command);
}
