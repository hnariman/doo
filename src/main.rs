use std::env;

fn create() { println!("create"); }
fn read()   { println!("read"); }
fn update() { println!("update"); }
fn delete() { println!("delete"); }
fn unknown() { println!("unknown"); }


fn main() {
    let args: Vec<String> = env::args().collect();
    // converting String into slice:
    let command = match &args[1][..]{
        "c" => create(),
        "r" => read(),
        "u" => update(),
        "d" => delete(),
        _ => unknown(),
    };

    // let result = match command { };

    // println!("command is : {:#?}",command);
}
