use crate::base;

pub struct Program {
    db: base::Storage,
}

impl Program {
    pub fn new() -> Self {
        Self {
            db: base::Storage::new("todo.db"),
        }
    }

    pub fn init_table(&self) {
        self.db.create_table();
    }

    pub fn create(&mut self, vars: &str) {
        // println!("create : {vars}");
        match vars.len() > 0 {
            true => self.db.create_item(vars),
            false => println!("creating empty string doesn't make sence though...")
        }
    }

    pub fn read(&self, vars: &str) {
        println!("read : {vars}");
    }

    pub fn update(&mut self, vars: &str) {
        println!("update : {vars}");
    }

    pub fn delete(&mut self, vars: &str) {
        // println!("delete : {vars}");
        self.db.delete_item(vars);
    }

    pub fn all(&mut self) {
        // println!("get all : {vars}");
        self.db.print_items();
    }

    pub fn help(&self) {
        println!("this is some help ");
    }

    pub fn unknown(&self) {
        println!("unknown argument, plase call --help");
    }
}
