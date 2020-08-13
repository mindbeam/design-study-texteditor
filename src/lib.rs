pub mod cursor;
pub mod document;
pub mod node;

#[cfg(test)]
mod test {

    use crate::{cursor::Cursor, document::Document};
    use std::sync::{Arc, Mutex};

    #[test]
    fn basic_deletion_and_regional_projection() {
        let document = Arc::new(Mutex::new(Document::new()));
        let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

        // (A) -> (BX) -> (delete X)
        //          \---> (C)
        cursor.insert("A".to_string());
        cursor.insert("BX".to_string());
        cursor.delete();
        cursor.insert("C".to_string());

        // assert_eq!(
        //     cursor.doc().diag(),
        //     "ⓧ (cf30=NULL), cf30↜(b50d=A @ 0), b50d↜(3f9d=BX @ 1), 3f9d↜(c799=␡ @ 2), 3f9d↜(dc27=C @ 1)"
        // );

        println!("{}", cursor.doc().diag_tree(Some(&cursor)));

        let proj = cursor.project_region(100);
        assert_eq!("ABC", proj);
    }

    #[test]
    fn multiple_deletes() {
        let document = Arc::new(Mutex::new(Document::new()));
        let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

        cursor.insert("A".to_string());
        cursor.insert("BX".to_string());
        cursor.delete();
        cursor.insert("C".to_string());
    }
}
