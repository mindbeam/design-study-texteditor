mod document;
mod node;

use document::Document;

fn main() {
    let mut doc = Document::new();

    let keys = vec!["M", "a", "r", "p", "\x08", "y"];

    for k in keys {
        match k {
            "\x08" => {
                // doc.remove(1);
            }
            _ => doc.insert(k.to_string()),
        }
    }

    println!(": {}", doc.render());
}
