use crate::{document::Document, node::NodeId};

#[derive(Clone)]
pub struct Cursor {
    pub node_id: NodeId,
    pub offset: u32,
}

impl Cursor {
    pub fn new(node_id: NodeId, offset: u32) -> Cursor {
        Cursor { node_id, offset }
    }
    pub fn back(&mut self, doc: &mut Document) {
        let updated = doc.traverse_left(self.node_id.clone(), 1, true);
        self.node_id = updated.0;
        self.offset = updated.1;
    }
}
