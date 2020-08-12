use crate::{
    document::Document,
    node::{Node, NodeId},
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Cursor {
    pub document: Arc<Mutex<Document>>,
    pub node_id: NodeId,
    pub offset: u32,
}

impl Cursor {
    pub fn new(document: Arc<Mutex<Document>>, node_id: NodeId, offset: u32) -> Cursor {
        Cursor {
            document,
            node_id,
            offset,
        }
    }
    pub fn doc(&self) -> std::sync::MutexGuard<Document> {
        self.document.lock().unwrap()
    }
    pub fn insert(&mut self, body: String) {
        let offset = body.len() as u32;

        let node = Node::insert(&self, body);
        let node_id = node.node_id();

        self.doc().add_node(node);

        self.node_id = node_id;
        self.offset = offset;
    }

    pub fn delete(&mut self) {
        let node = Node::delete(&self);

        self.doc().add_node(node);
        self.left(1);
    }

    pub fn left(&mut self, mut positions: u32) {
        // the cursor has an offset to it's referenced node
        // that node has an offset to its parent

        // first, lets see if we can do this without changing nodes
        if positions < self.offset {
            // We don't want to go to zero
            self.offset - positions;
            return;
        }

        // So we have to change nodes. Adjust the number of positions
        // to reflect a zero offset to the referenced node
        positions -= self.offset;

        let updated = self
            .doc()
            .traverse_left(self.node_id.clone(), positions, true);
        self.node_id = updated.0;
        self.offset = updated.1;
    }
}
