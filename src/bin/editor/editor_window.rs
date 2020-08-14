use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, OnEventView, TextArea};
use cursive::Cursive;
use keystroke_crdt_experiment::document::Document;

use crate::editor::Editor;

pub struct EditorWindow {
    document: Document,
}

impl EditorWindow {
    pub fn new(document: Document) -> Self {
        EditorWindow { document }
    }
    pub fn run(&self) {
        let mut siv = cursive::default();

        // The main dialog will just have a textarea.
        // Its size expand automatically with the content.

        let editor = Editor::new(&self.document).fixed_width(30);

        siv.add_layer(editor);

        // bind_find(siv);

        siv.run();
    }
}

// fn bind_find(siv: &Cursive) {
//     // We'll add a find feature!
//     siv.add_layer(Dialog::info("Hint: press Ctrl-F to find in text!"));

//     siv.add_global_callback(Event::CtrlChar('f'), |s| {
//         // When Ctrl-F is pressed, show the Find popup.
//         // Pressing the Escape key will discard it.
//         s.add_layer(
//             OnEventView::new(
//                 Dialog::new()
//                     .title("Find")
//                     .content(
//                         EditView::new()
//                             .on_submit(find)
//                             .with_name("edit")
//                             .min_width(10),
//                     )
//                     .button("Ok", |s| {
//                         let text = s
//                             .call_on_name("edit", |view: &mut EditView| view.get_content())
//                             .unwrap();
//                         find(s, &text);
//                     })
//                     .dismiss_button("Cancel"),
//             )
//             .on_event(Event::Key(Key::Esc), |s| {
//                 s.pop_layer();
//             }),
//         )
//     });
// }

// fn find(siv: &mut Cursive, text: &str) {
//     // First, remove the find popup
//     siv.pop_layer();

//     let res = siv.call_on_name("text", |v: &mut TextArea| {
//         // Find the given text from the text area content
//         // Possible improvement: search after the current cursor.
//         if let Some(i) = v.get_content().find(text) {
//             // If we found it, move the cursor
//             v.set_cursor(i);
//             Ok(())
//         } else {
//             // Otherwise, return an error so we can show a warning.
//             Err(())
//         }
//     });

//     if let Some(Err(())) = res {
//         // If we didn't find anything, tell the user!
//         siv.add_layer(Dialog::info(format!("`{}` not found", text)));
//     }
// }
