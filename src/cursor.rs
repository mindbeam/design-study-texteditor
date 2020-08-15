use crate::{
    document::{Document, DocumentInner},
    node::{Node, NodeId},
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Cursor {
    pub document: Document,
    pub node_id: NodeId,
    pub offset: usize,
}

impl Cursor {
    pub fn root(document: &Document) -> Cursor {
        Cursor {
            document: document.clone(),
            node_id: document.inner().root_node(),
            offset: 0,
        }
    }
    pub fn doc(&self) -> std::sync::MutexGuard<DocumentInner> {
        self.document.inner()
    }
    pub fn insert(&mut self, body: String) {
        let offset = body.len();

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
    pub fn left(&mut self, mut positions: usize) {
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

    // Look leftward by up to rewind_by_chars and then project the tree forward
    pub fn rewind_and_project(
        &self,
        rewind_by_chars: usize,
        character_limit: Option<usize>,
    ) -> String {
        let mut rewind = self.clone();
        rewind.left(rewind_by_chars);

        rewind.project_forward(character_limit)
    }

    /// Project a string from the document, starting at the current cursor position
    pub fn project_forward(&self, character_limit: Option<usize>) -> String {
        // Todo make this an iterator
        struct Hop {
            node_id: NodeId,
            node: Node,
            render_offset: usize,
        }
        let mut buf = String::new();

        let mut hopper: Vec<Hop> = Vec::new();

        let doc = self.doc();
        let node = doc
            .nodes
            .get(&self.node_id)
            .expect("Node not found")
            .clone();

        hopper.push(Hop {
            node_id: self.node_id.clone(),
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
