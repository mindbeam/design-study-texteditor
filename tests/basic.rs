use keystroke_crdt_experiment::{cursor::Cursor, document::Document};
use std::sync::{Arc, Mutex};

#[test]

fn marpy() {
    let document = Arc::new(Mutex::new(Document::new()));
    let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

    for k in "Marp\x08y hab\x08d a liffle\x08\x08\x08\x08ttle lab".chars() {
        match k {
            '\x08' => cursor.delete(),
            _ => cursor.insert(k.to_string()),
        }
    }

    cursor.left(1);
    cursor.insert("m".to_string());

    println!("{}", cursor.doc().diag_tree(Some(&cursor)));
    assert_eq!("Mary had a little lamb", cursor.project_region(100));
}
