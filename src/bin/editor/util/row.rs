use keystroke_crdt_experiment::{cursor::Cursor, node::NodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The NodeId for the first character of the row
    pub start_node_id: NodeId,

    /// Offset into the start_node_id for the first character of the row.
    pub start_offset: usize,

    /// The NodeId for the last character of the row
    pub end_node_id: NodeId,

    /// Offset into the end_node_id for the last character of the row, exclusive
    pub end_offset: usize,

    /// Width of the row, in cells.
    pub width: usize,

    /// Whether or not this text was wrapped onto the next line
    pub is_wrapped: bool,
}

pub fn make_rows(cursor: Cursor, width: usize) -> Vec<Row> {
    // We can't make rows with width=0, so force at least width=1.
    let width = usize::max(width, 1);
    unimplemented!()
}

struct RowIterator {
    cursor: Cursor,
    max_width: usize,
}

impl RowIterator {
    pub fn new(cursor: Cursor, max_width: usize) -> Self {
        RowIterator {
            cursor,
            max_width,
            // next_row_buf: String::new(),
        }
    }
}

impl Iterator for RowIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        // let buf = self.cursor.scan_forward(Some(self.max_width));

        let mut last_node_id = self.cursor.node_id.clone();
        let mut buf = String::new();

        // self.cursor
        //     .scan_forward(|node_id, node, render_offset, children| {
        //         node.project(&mut buf, render_offset);

        //         last_node_id = node_id.clone();
        //         // if we have children, they might be deletions, so we have to keep going
        //         if let Some(_) = children {
        //             return true;
        //         }

        //         if let Some(offset) = node.find("\n") {
        //             // TODO - determine if there are any deletions before this character
        //         }
        //         if buf.len() >= limit {
        //             buf.truncate(limit);
        //             false
        //         } else {
        //             true
        //         }
        //     });

        unimplemented!()

        // Row {
        //     start_node_id: self.cursor.node_id,
        //     start_offset: (),
        //     // end_node_id: (),
        //     // end_offset: (),
        //     width: buf.len(),
        //     // is_wrapped: (),
        // }
    }
}

// impl Row {
//     /// Shift a row start and end by `offset`.
//     pub fn shift(&mut self, offset: usize) {
//         self.start += offset;
//         self.end += offset;
//     }

//     /// Shift a row start and end by `offset`.
//     ///
//     /// Chainable variant;
//     pub fn shifted(self, offset: usize) -> Self {
//         self.with(|s| s.shift(offset))
//     }

//     /// Shift back a row start and end by `offset`.
//     pub fn rev_shift(&mut self, offset: usize) {
//         self.start -= offset;
//         self.end -= offset;
//     }
// }
