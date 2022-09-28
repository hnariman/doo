// imports 
use rusqlite::{Connection};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};

// structs
#[derive(Debug, Default, Table)]
pub struct Item {
    #[table(title="ID", justify="Justify::Center")]
    pub id: i32,
    #[table(title="Note", justify="Justify::Center")]
    pub note: String,
    #[table(title="Done", justify="Justify::Center")]
    pub done: bool
}

// #[derive(Debug, Default)]
pub struct Storage {
    connection:Connection,
    items:Vec<Item>
}

impl Storage {
    pub fn new(name:&str) -> Self {
        Self {
            connection :Connection::open(name).unwrap(),
            items:vec![]
        }
    }

    // craete sqlite table
    pub fn create_table(&self){
        self.connection.execute(
            "create table if not exists todo (
           id integer primary key,
           note text not null,
           done boolean
        )",(),
        ).unwrap();
    }

    // create todo item
    pub fn create_item(&self, note:&str) {
        let mut item = Item::default();
        item.note = note.to_string();
        self.connection.execute("INSERT INTO todo (note, done) VALUES (?1, ?2)", (&item.note, &item.done)).unwrap();
    }

    // get all todo items
    pub fn get_all_items(&mut self) {
        let mut query = self.connection.prepare("SELECT id, note, done FROM todo").unwrap();

        let rows = query.query_map([], |row| {
            Ok(Item{
                id:row.get(0)?,
                note:row.get(1)?,
                done:row.get(2)?,
            })
        }).unwrap();

        let mut result : Vec<Item> = Vec::new();

        for row in rows {
            result.push(row.unwrap());
        }
        self.items = result
    }

    pub fn print_items(&mut self){
        let _ = &mut self.get_all_items();
        print_stdout(self.items.with_title()).expect("unable to print table");
        // for item in &self.items {
        //     let Item{id, note, done} = item;
        //     println!("{}. {} - {}", id, note, done);
        // }
    }
}

