use keystroke_crdt_experiment::cursor::Cursor;
use keystroke_crdt_experiment::document::Document;
use std::io::{self, stdout};
use std::sync::{Arc, Mutex};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let document = Arc::new(Mutex::new(Document::new()));
    let mut cursor = Cursor::new(document.clone(), document.lock().unwrap().root_node(), 0);

    for key in io::stdin().keys() {
        let key = key.unwrap();
        match key {
            Key::Ctrl('q') | Key::Ctrl('c') => break,
            Key::Backspace => cursor.delete(),
            Key::Char(c) => {
                if c.is_control() {
                    // println!("{:?}\r", c as u8);
                } else {
                    // println!("{:?} ({})\r", c as u8, c);
                    cursor.insert(c.to_string());
                };
            }
            _ => {}
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
