// imports 
use rusqlite::{Connection, params};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};

// structs
#[derive(Debug, Default, Table)]
pub struct Item {
    pub id: i32,
    pub note: String,
    pub done: bool
}

#[derive(Debug,Table)]
pub struct PrintItem {
    #[table(title="ID", justify="Justify::Center")]
    pub id: i32,
    #[table(title="Note", justify="Justify::Left")]
    pub note: String,
    #[table(title="Done", justify="Justify::Center")]
    pub done: String
}

impl From<&Item> for PrintItem {
    fn from(item:&Item) -> PrintItem {
       PrintItem {
            id:item.id,
            note: item.note.clone(),
            done: if item.done {String::from("[+]")} else { String::from("[]") }
        }
    }
}

// #[derive(Debug, Default)]
/// Storage entity
pub struct Storage {
    connection:Connection,
    items:Vec<Item>
}

// basic sql
const CREATE_TODO: &str= 
"create table if not exists todo (
    id integer primary key,
    note text not null,
    done boolean not null
)";
const INSERT_TODO: &str = "INSERT INTO todo (note, done) VALUES (?1, ?2)";
const SELECT_TODO: &str = "SELECT id, note, done FROM todo";
const DELETE_TODO: &str = "DELETE FROM todo WHERE id = ?";


impl Storage {
    pub fn new(name:&str) -> Self {
        Self {
            connection :Connection::open(name).unwrap(),
            items:vec![]
        }
    }

    // craete sqlite table
    pub fn create_table(&self){
        self.connection.execute(CREATE_TODO,(),).unwrap();
    }

    // create todo item
    pub fn create_item(&self, note:&str) {
        let mut item = Item::default();
        item.note = note.to_string();
        self.connection.execute(INSERT_TODO, (&item.note, &item.done)).unwrap();
    }

    // get all todo items
    pub fn get_all_items(&mut self) {
        let mut query = self.connection.prepare(SELECT_TODO).unwrap();

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

    pub fn delete_item(&self, id:&str) {
        self.connection.execute(DELETE_TODO,params![id]).unwrap();
    }

    pub fn print_items(&mut self){
        let _ = &mut self.get_all_items();
        // here is a tricky convertsion of Map.iter into vec
        let print_items = self.items.iter().map(|item| PrintItem::from(item)).collect::<Vec<_>>();

        print_stdout(print_items.with_title()).expect("unable to print table");
    }
}

