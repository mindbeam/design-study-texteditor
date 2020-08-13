use keystroke_crdt_experiment::{cursor::Cursor, document::Document};
use std::sync::{Arc, Mutex};

/// Each child node should be unique based on parent, clock, and content
#[test]
fn repeated_child_node() {
    let document = Arc::new(Mutex::new(Document::new()));
    let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

    cursor.insert("a".to_string());
    let first_a = cursor.node_id.clone();
    cursor.delete();
    cursor.insert("a".to_string());
    let second_a = cursor.node_id.clone();

    // If this fails, that means the second "a" child of ROOT collided with the first
    assert_ne!(first_a, second_a);

    cursor.insert("b".to_string());

    // println!("{}", cursor.doc().diag());
    println!("{}", cursor.doc().diag_tree(Some(&cursor)));

    let s = cursor.project_region(100);
    assert_eq!("ab", s);
}

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
