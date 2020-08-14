use keystroke_crdt_experiment::{cursor::Cursor, document::Document};

mod editor;
mod editor_window;
use editor_window::EditorWindow;
use std::sync::{Arc, Mutex};

use std::{fs::File, io::BufRead, io::BufReader, path::PathBuf};
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    match opt.file {
        Some(path) => {
            let display = path.display();

            let file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };

            let buf_reader = BufReader::new(file);

            let document = Document::new();
            let mut cursor = Cursor::root(&document);

            for line in buf_reader.lines() {
                let line = line.unwrap();
                cursor.insert(line);
            }

            // println!("{}", cursor.doc().diag_tree(Some(&cursor)));

            let editor = EditorWindow::new(document);
            editor.run();
        }
        None => {}
    }
}
