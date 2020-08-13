use io::stdout;
use keystroke_crdt_experiment::cursor::Cursor;
use keystroke_crdt_experiment::document::Document;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let document = Arc::new(Mutex::new(Document::new()));
    let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        // println!("{}", c);
        match c {
            '\x08' => {
                cursor.delete();
            }
            'q' => break,
            _ => cursor.insert(c.to_string()),
        }

        print!("{}\r\n", cursor.project_region(100));
    }

    // cursor.left(1);
    // cursor.insert("m".to_string());

    // cursor.left(9);
    // println!(": {}", cursor.project_region(100));

    // PAUSE - you're processing this in the wrong fucking order. should be LABM not LAMB
    // figure out the clock, then outward/inward rendered offsettting, and idempotence
}
