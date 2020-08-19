use tracing::debug;

use crate::util::row::Row;
use cursive::{
    event::{Event, EventResult, Key},
    Printer, Vec2, View,
};
use design_study_texteditor::{cursor::Cursor, document::Document};

pub struct Editor {
    page_cursor: Cursor,
    edit_cursor: Cursor,

    rows: Vec<Row>,
    // last known size of the renderable text region
    last_size: Vec2,
}

impl Editor {
    pub fn new(document: &Document) -> Self {
        let page_cursor = Cursor::root(document);
        let mut edit_cursor = page_cursor.clone();
        edit_cursor.offset = 2;

        Editor {
            page_cursor,
            edit_cursor,
            last_size: Vec2::zero(),
            rows: Vec::new(),
        }
    }

    fn insert(&mut self, ch: char) {
        self.edit_cursor.insert(ch.to_string());

        self.compute_rows(self.last_size);
    }

    fn compute_rows(&mut self, size: Vec2) {
        // if self.is_cache_valid(size) {
        //     debug!("Cache is still valid.");
        //     return;
        // }
        debug!("Computing rows");

        let mut available = size.x;

        // self.rows = crate::util::row::make_rows(self.page_cursor, available);
        // self.fix_ghost_row();

        // if self.rows.len() > size.y {
        //     available = available.saturating_sub(1);
        //     // Apparently we'll need a scrollbar. Doh :(
        //     self.rows = make_rows(&self.content, available);
        //     self.fix_ghost_row();
        // }

        // if !self.rows.is_empty() {
        //     self.size_cache = Some(SizeCache::build(size, size));
        // }
    }
}

impl View for Editor {
    fn layout(&mut self, size: Vec2) {
        self.last_size = size;
        self.compute_rows(size);
    }
    fn draw(&self, printer: &Printer) {
        printer.print(
            (0, 0),
            &self.page_cursor.project_forward(Some(self.last_size.x)),
        )

        // TODO line wrapping
        // TODO scrolling
        // TODO presence indication (cursor location)
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(ch) => self.insert(ch),
            Event::Key(Key::Backspace) => self.edit_cursor.delete(),
            Event::Key(Key::Left) => self.edit_cursor.left(1),
            Event::Key(Key::Right) => self.edit_cursor.right(1),
            _ => {}
        };
        EventResult::Consumed(None)
    }
}
