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

    /// Move the cursor N positions to the left within the document
    /// Note that the cursor itself may have an offset to the referenced node
    /// Which is conceptually similar to the cursor itself being like a node
    // referencing a parent node
    pub fn left(&mut self, mut positions: u32) {
        // first, lets see if we can do this without changing nodes
        if positions < self.offset {
            // We don't want to go to zero
            self.offset -= positions;
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

    // Project a number of characters before and after
    pub fn project_region(&self, chars_before_and_after: u32) -> String {
        // TODO - make nodes BTreeMap<NodeId,Vec<NodeUnit>>
        // NO! Do it from child to parent after all
        // this will allow scrolling to render a partial document. start with the cursor

        let mut buf = String::new();

        struct Hop {
            node_id: NodeId,
            node: Node,
            render_offset: u32,
        }

        let mut hopper: Vec<Hop> = Vec::new();

        let mut left = self.clone();
        left.left(chars_before_and_after);

        let doc = self.doc();
        let node = doc
            .nodes
            .get(&left.node_id)
            .expect("Node not found")
            .clone();

        hopper.push(Hop {
            node_id: left.node_id.clone(),
            node,
            render_offset: 0,
        });

        while let Some(hop) = hopper.pop() {
            let node = doc.nodes.get(&hop.node_id).expect("Node not found");

            node.project(&mut buf, hop.render_offset);

            let mut subhopper = Vec::new();
            if let Some(children) = doc.child_map.get(&hop.node_id) {
                for child_id in children {
                    let child = doc.nodes.get(&child_id).expect("Node not found").clone();

                    let render_offset = hop.render_offset + child.offset();

                    subhopper.push(Hop {
                        node_id: child_id.clone(),
                        node: child,
                        render_offset,
                    })
                }
            }

            // Reverse sort because we're conusming the tail *facepalm*
            subhopper.sort_by(|a, b| b.node.tick.cmp(&a.node.tick));

            hopper.extend(subhopper);
        }

        buf
    }
}
