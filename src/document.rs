use crate::cursor::Cursor;
use crate::node::{Node, NodeId};
use std::collections::{BTreeMap, VecDeque};

pub struct Document {
    nodes: BTreeMap<NodeId, Node>,
    child_map: BTreeMap<NodeId, Vec<NodeId>>,
    clock: u32,
    root: NodeId,
}

impl Document {
    pub fn new() -> Document {
        let root = Node::root(0);
        let node_id = root.node_id();

        let mut me = Document {
            root: node_id,
            clock: 1,
            nodes: BTreeMap::new(),
            child_map: BTreeMap::new(),
        };

        me.add_node(root);

        me
    }
    pub fn root_node(&self) -> NodeId {
        self.root.clone()
    }

    pub fn get_node(&self, node_id: &NodeId) -> &Node {
        self.nodes.get(node_id).expect("Node not found")
    }
    pub fn increment_clock(&mut self) -> u32 {
        let reading = self.clock;
        self.clock += 1;
        reading
    }

    /// Traverse left a given number of positions from the zero offset of a given node
    pub fn traverse_left(
        &self,
        mut node_id: NodeId,
        mut positions: u32,
        try_avoid_zero: bool,
    ) -> (NodeId, u32) {
        //
        while positions > 0 {
            let node = self.nodes.get(&node_id).expect("Node not found");
            match &node.parent {
                Some(parent) => {
                    let offset = node.offset();
                    if !try_avoid_zero && offset > positions {
                        return (node_id.clone(), offset - positions);
                    } else {
                        positions -= offset;
                        node_id = parent.clone();
                        // Keeep goinggg
                    }
                }
                None => return (node_id.clone(), 0),
            }
        }
        return (node_id.clone(), 0);
    }
    pub fn add_node(&mut self, node: Node) {
        let node_id = node.node_id();

        // Look up the parent for this node and get it's calculated offset
        // That will become out calculated_parent_offset
        if let Some(parent) = &node.parent {
            use std::collections::btree_map::Entry;

            // register this as a child of its parent
            match self.child_map.entry(parent.clone()) {
                Entry::Vacant(e) => {
                    e.insert(vec![node_id.clone()]);
                }
                Entry::Occupied(mut e) => {
                    let v = e.get_mut();
                    // match v.binary_search(&node_id) {
                    //     Ok(_) => {}
                    //     Err(i) => v.insert(i, node_id.clone()),
                    // }
                    if !v.contains(&node_id) {
                        v.push(node_id.clone())
                    }
                }
            }
        };

        self.nodes.insert(node_id, node);
    }
    pub fn render(&self, cursor: &Cursor) -> String {
        // TODO - make nodes BTreeMap<NodeId,Vec<NodeUnit>>
        // NO! Do it from child to parent after all
        // this will allow scrolling to render a partial document. start with the cursor

        let mut buf = String::new();

        let Cursor {
            node_id, offset, ..
        } = cursor;

        struct Hop {
            node_id: NodeId,
            render_offset: u32,
        }

        let mut hopper: Vec<Hop> = Vec::new();
        hopper.push(Hop {
            node_id: node_id.clone(),
            render_offset: 0,
        });

        fn raise(hopper: &mut Vec<Hop>, gt: u32, add: u32) {
            for hop in hopper.iter_mut() {
                use std::cmp::Ordering::*;
                match hop.render_offset.cmp(&gt) {
                    Less => {}
                    Equal | Greater => {
                        hop.render_offset += add;
                    }
                }
            }
        };

        while let Some(hop) = hopper.pop() {
            let node = self.nodes.get(&hop.node_id).expect("Node not found");
            // if let Some(parent) = &nu.node.parent {
            //     hopper.push(Inward(parent.clone()));
            // }
            node.materialize(&mut buf, hop.render_offset);
            if let Some(children) = self.child_map.get(&hop.node_id) {
                for child_id in children {
                    let child = self.nodes.get(&child_id).expect("Node not found");

                    let render_offset = hop.render_offset + child.offset();
                    raise(&mut hopper, render_offset, child.offset());

                    hopper.push(Hop {
                        node_id: child_id.clone(),
                        render_offset,
                    })
                }
            }
        }

        buf
    }
    #[allow(unused)]
    pub fn diag(&self) -> String {
        let mut out = String::new();
        use std::fmt::Write;

        let mut v: Vec<(&NodeId, &Node)> = self.nodes.iter().collect();
        v.sort_by(|a, b| a.1.tick.cmp(&b.1.tick));
        let mut delim = false;
        for (node_id, node) in v {
            if delim {
                write!(out, ", ").unwrap();
            }
            delim = true;
            match &node.parent {
                Some(parent) => write!(
                    out,
                    "{}↜({}={})",
                    parent.hex4(),
                    node_id.hex4(),
                    node.diag()
                )
                .unwrap(),
                None => write!(out, "ⓧ ({}={})", node_id.hex4(), node.diag()).unwrap(),
            }
        }
        out
    }

    #[allow(unused)]
    pub fn diag_tree(&self, cursor: Option<&Cursor>) -> String {
        let mut out = String::new();
        use std::fmt::Write;

        struct Hop {
            node_id: NodeId,
            node: Node,
            tier: usize,
        }

        let mut hopper: Vec<Hop> = Vec::new();

        let node = self.nodes.get(&self.root).expect("Node not found");

        hopper.push(Hop {
            node_id: self.root.clone(),
            node: node.clone(),
            tier: 0,
        });

        while let Some(hop) = hopper.pop() {
            if hop.tier > 0 {
                for _ in std::iter::repeat("\t").take(hop.tier - 1) {
                    write!(out, "      ").unwrap();
                }
                write!(out, "    \\-");
            }
            write!(out, "{}: {}\n", hop.node_id.hex4(), hop.node.diag(),).unwrap();
            if let Some(cursor) = cursor {
                if cursor.node_id == hop.node_id {
                    for _ in std::iter::repeat("\t").take(hop.tier - 1) {
                        write!(out, "      ").unwrap();
                    }
                    write!(out, "      ^ CURSOR @ {}\n", cursor.offset);
                }
            }

            let mut subhopper = Vec::new();
            if let Some(children) = self.child_map.get(&hop.node_id) {
                for child_id in children {
                    let child = self.nodes.get(&child_id).expect("Node not found");

                    subhopper.push(Hop {
                        node_id: child_id.clone(),
                        node: child.clone(),
                        tier: hop.tier + 1,
                    })
                }
            }

            // Reverse sort because we're conusming the tail *facepalm*
            subhopper.sort_by(|a, b| b.node.tick.cmp(&a.node.tick));

            hopper.extend(subhopper);
        }

        out
    }
}
