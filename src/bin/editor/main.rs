use design_study_texteditor::{cursor::Cursor, document::Document};

mod editor;
mod editor_window;
mod util;
use editor_window::EditorWindow;

use std::{fs::File, io::BufRead, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use tracing::info;

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
            design_study_texteditor::util::logging::init_logging();
            openfile(path);
        }
        None => {}
    }
}

fn openfile(path: PathBuf) {
    let display = path.display();
    let bullshit = &display; //.to_string();
    info!("Opening file {}", bullshit);

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
