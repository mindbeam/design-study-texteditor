mod cursor;
mod document;
mod node;

use cursor::Cursor;
use document::Document;
use std::sync::{Arc, Mutex};

fn main() {
    let document = Arc::new(Mutex::new(Document::new()));
    let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

    let keys: Vec<char> = "Marp\x08y had a little lab".chars().collect();

    for k in keys {
        match k {
            '\x08' => {
                cursor.delete();
            }
            _ => cursor.insert(k.to_string()),
        }
    }

    println!(": {}", document.lock().unwrap().render(&cursor));

    cursor.left(1);
    cursor.insert("m".to_string());

    println!(": {}", document.lock().unwrap().render(&cursor));
}
