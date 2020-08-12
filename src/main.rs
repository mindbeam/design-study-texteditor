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

    cursor.left(9);
    println!(": {}", document.lock().unwrap().render(&cursor));

    // PAUSE - you're processing this in the wrong fucking order. should be LABM not LAMB
    // figure out the clock, then outward/inward rendered offsettting, and idempotence
}

#[cfg(test)]
mod test {

    use crate::{cursor::Cursor, document::Document};
    use std::sync::{Arc, Mutex};

    #[test]
    fn traversal1() {
        let document = Arc::new(Mutex::new(Document::new()));
        let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

        // (A) -> (BX) -> (delete X)
        //          \---> (C)
        cursor.insert("A".to_string());
        cursor.insert("BX".to_string());
        cursor.delete();
        cursor.insert("C".to_string());
        assert_eq!(
            cursor.doc().diag(),
            "ⓧ (cf30=NULL), cf30↜(b50d=A @ 0), b50d↜(3f9d=BX @ 1), 3f9d↜(c799=␡ @ 2), b50d↜(dcd0=C @ 0)"
        );

        println!("{}", cursor.doc().diag_tree(Some(&cursor)));

        // seems to be busylooping
        cursor.left(1);

        println!("{}", cursor.doc().diag_tree(Some(&cursor)));
        // println!(": {}", document.lock().unwrap().render(&cursor));
    }
}
