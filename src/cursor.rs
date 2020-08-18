use crate::{
    document::{Document, DocumentInner},
    node::{Node, NodeId},
    util::mutstr::MutStr,
};

#[derive(Clone)]
pub struct Cursor {
    pub document: Document,
    pub node_id: NodeId,
    pub offset: usize,
}

pub struct NodeCursor {
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

    /// Project a string from the document, starting at the current cursor position
    pub fn project_forward(&self, mut character_limit: Option<usize>) -> String {
        let mut buf = String::new();

        self.depth_first_scan(|_node_id, node, render_offset, children| {
            // println!("{:?}", character_limit);

            let mut slice = MutStr::slice(&mut buf, render_offset..);
            let len = node.project(&mut slice, character_limit);

            if let Some(limit) = character_limit.as_mut() {
                if len >= 0 {
                    *limit -= len as usize;
                } else {
                    *limit += len as usize;
                }

                // *limit > 0
                // } else {
                // true
            }

            // if let Some(_) = children {
            //     true
            // }
        });

        buf
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

    pub fn depth_first_scan<F>(&self, mut f: F)
    where
        F: FnMut(&NodeId, &Node, usize, Option<&Vec<NodeId>>),
    {
        // Scan through the node tree, in a depth-first fashion.
        // We are trying to avoid stack-based recursion, as we may be processing millions of nodes
        //
        // Rules:
        // 0. All non-concurrent edits are:
        //    * children of their preceeding edit node
        //    * causally descendant of same (And of prior observed concurrencies?)
        //    * temporally descendant according to the logical clock
        // 1. Each child node may only "edit" its direct parent (within its range, as modified by preceding operations)
        //     * The parent offset may shift, which should be harmless
        //     * The problem is that parent length may also shift, which is NOT harmless
        // 2. Children must always be causally descendant of their parent (and thus posess a later clock reading)
        // 3. Each child node may only edit within the range of its direct parent (as modified by)

        // QUESTIONS:
        // 1. How do we contend with transformation of concurrently applied (sibling) operations?

        struct Hop {
            node_id: NodeId,
            node: Node,
            parent_offset: usize,
        }

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
            parent_offset: 0,
        });

        while let Some(hop) = hopper.pop() {
            let node = doc.nodes.get(&hop.node_id).expect("Node not found");

            let children = doc.child_map.get(&hop.node_id);
            f(&hop.node_id, node, hop.parent_offset, children);
            let parent_offset = hop.parent_offset + node.offset();

            let mut subhopper = Vec::new();
            if let Some(children) = children {
                for child_id in children {
                    let child = doc.nodes.get(&child_id).expect("Node not found").clone();

                    subhopper.push(Hop {
                        node_id: child_id.clone(),
                        node: child,
                        parent_offset,
                    })
                }
            }

            // Reverse sort because we're conusming the tail *facepalm*
            subhopper.sort_by(|a, b| b.node.tick.cmp(&a.node.tick));

            hopper.extend(subhopper);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{cursor::Cursor, document::Document};

    #[test]
    fn limit() {
        crate::util::logging::init_logging();
        let document = Document::new();
        let mut cursor = Cursor::root(&document);

        cursor.insert("ABC".to_string());
        cursor.insert("DEF".to_string());
        cursor.delete();
        cursor.insert("GHI".to_string());

        let proj = cursor.rewind_and_project(100, Some(4));
        assert_eq!("ABCD", proj);

        // println!("{}", cursor.doc().diag_tree(Some(&cursor)));

        let proj = cursor.rewind_and_project(100, Some(6));
        assert_eq!("ABCDEG", proj);
    }

    #[test]
    fn limit_2() {
        crate::util::logging::init_logging();
        let document = Document::new();
        let mut cursor = Cursor::root(&document);

        cursor.insert("ABC".to_string());
        cursor.insert("DEF".to_string());
        cursor.delete();
        cursor.delete();
        cursor.delete();
        cursor.delete();
        cursor.insert("GHI".to_string());

        let proj = cursor.rewind_and_project(100, Some(3));
        assert_eq!("ABG", proj);
    }

    // #[test]
    fn incremental_scan() {
        let document = Document::new();
        let mut cursor = Cursor::root(&document);

        // (A) -> (BX) -> (delete X)
        //          \---> (C)
        cursor.insert("ABC".to_string());
        cursor.insert("DEF".to_string());
        cursor.delete();
        cursor.insert("GHI".to_string());

        // println!("{}", cursor.doc().diag_tree(Some(&cursor)));

        let cursor = Cursor::root(&document);

        // let mut out = Vec::new();
        let mut count = 0;

        cursor.depth_first_scan(|node_id, node, render_offset, children| {
            // println!("{}, {}: {}", node_id.hex4(), render_offset, node.diag());

            // In keeping with the stupidest-way doctrine, lets scan through the children here.
            // The question is: Do we do this ourselves? or do we

            // true
            // match children {
            //     Some(children) => {}
            //     None => {}
            // }
            // unimplemented!()
        });

        // assert_eq!("ABC", proj);
    }
}

// Only immediate child nodes may insert or delete into a node
// Grandchild nodes may only modify those modifications

// So what can we do to apply child nodes incrementally?
// It's really about the clock. In order to achive this, we need to look at the maximum clock reading, indexed by minimum offset
// And then play forward the operations in clock order until the minimum reading is cleared

// Of course the simplest thing to do is to just play all child nodes recursively, but given that we're starting at the root,
// that's essentially the same as projecting the full document.
