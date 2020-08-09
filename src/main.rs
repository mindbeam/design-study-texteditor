mod cursor;
mod document;
mod node;

use document::Document;

fn main() {
    let mut doc = Document::new();

    let keys: Vec<char> = "Marp\x08y had a little lamb".chars().collect();

    for k in keys {
        match k {
            '\x08' => {
                doc.delete();
            }
            _ => doc.insert(k.to_string()),
        }
    }

    println!(": {}", doc.render());
}
