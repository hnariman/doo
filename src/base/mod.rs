// imports 
use rusqlite::{Connection};

// structs
#[derive(Debug, Default)]
pub struct Item {
    pub id: i32,
    pub note: String,
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
        for item in &self.items {
            let Item{id, note, done} = item;
            println!("{}. {} - {}", id, note, done);
        }
    }
}

// pub fn init() -> &'static Connection{
//     let db:Connection = Connection::open("items.db").unwrap();
//     &db
// }

// // craete sqlite table
// pub fn create_table(){
//     let db = init();
//     db.execute(
//         "create table if not exists todo (
//            id integer primary key,
//            note text not null,
//            done boolean
//         )",(),
//     ).unwrap();
// }

// // create todo item
// pub fn create_item(note:&str) {
//     let db = init();

//     let mut item = Item::default();
//     item.note = note.to_string();
//     db.execute("INSERT INTO todo (note, done) VALUES (?1, ?2)", (&item.note, &item.done)).unwrap();
// }

// // get all todo items
// pub fn get_all_items() -> Vec<Item>{
//     let db = init();
//     let mut query = db.prepare("SELECT id, note, done FROM todo").unwrap();

//     let rows = query.query_map([], |row| {
//         Ok(Item{
//             id:row.get(0)?,
//             note:row.get(1)?,
//             done:row.get(2)?,
//         })
//     }).unwrap();

//     let mut result : Vec<Item> = Vec::new();

//     for row in rows {
//         result.push(row.unwrap());
//     }
//     result
// }

// pub fn print_rows(rows: Vec<Item>){
//     let db = init();
//     let items = get_all_items();

//     for item in items {
//         let Item{id, note, done} = item;
//         println!("{}. {} - {}", id, note, done);
//     }

// }

