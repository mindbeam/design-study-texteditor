use cursive::{Printer, Vec2, View};
use keystroke_crdt_experiment::{cursor::Cursor, document::Document};

pub struct Editor {
    cursor: Cursor,
}

impl Editor {
    pub fn new(document: &Document) -> Self {
        Editor {
            cursor: Cursor::root(document),
        }
    }
}

impl View for Editor {
    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // Make sure our structure is up to date
        // self.soft_compute_rows(constraint);

        // // Ideally, we'd want x = the longest row + 1
        // // (we always keep a space at the end)
        // // And y = number of rows
        // debug!("{:?}", self.rows);
        // let scroll_width = if self.rows.len() > constraint.y { 1 } else { 0 };

        // let content_width = if self.rows.iter().any(|row| row.is_wrapped) {
        //     // If any row has been wrapped, we want to take the full width.
        //     constraint.x.saturating_sub(1 + scroll_width)
        // } else {
        //     self.rows.iter().map(|r| r.width).max().unwrap_or(1)
        // };

        // Vec2::new(scroll_width + 1 + content_width, self.rows.len())
        Vec2::new(30, 30)
    }
    fn draw(&self, printer: &Printer) {
        printer.print((0, 0), "Meow")
    }
}
