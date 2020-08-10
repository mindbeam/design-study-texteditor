use crate::cursor::Cursor;
use crate::node::{Node, NodeId};
use std::collections::BTreeMap;

pub struct Document {
    nodes: BTreeMap<NodeId, NodeUnit>,
    root: NodeId,
}

struct NodeUnit {
    node: Node,
    calculated_parent_offset: u32,
}

impl NodeUnit {
    fn calculated_offset(&self) -> u32 {
        self.calculated_parent_offset + self.node.offset()
    }
}

impl Document {
    pub fn new() -> Document {
        let root = Node::root();
        let node_id = root.node_id();

        let mut nodes: BTreeMap<NodeId, NodeUnit> = BTreeMap::new();
        let unit = NodeUnit {
            node: root,
            calculated_parent_offset: 0,
        };

        nodes.insert(node_id.clone(), unit);
        Document {
            root: node_id,
            nodes,
        }
    }
    pub fn root_node(&self) -> NodeId {
        self.root.clone()
    }

    pub fn get_node(&self, node_id: &NodeId) -> &Node {
        &self.nodes.get(node_id).expect("Node not found").node
    }

    pub fn traverse_left(
        &self,
        mut node_id: NodeId,
        mut positions: u32,
        try_avoid_zero: bool,
    ) -> (NodeId, u32) {
        while positions > 0 {
            let nu = self.nodes.get(&node_id).expect("Node not found");
            match &nu.node.parent {
                Some(parent) => {
                    let offset = nu.node.offset();
                    if (!try_avoid_zero && positions >= offset) || (positions > offset) {
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
        let calculated_parent_offset = match &node.parent {
            Some(parent) => {
                let p_nu = self.nodes.get(parent).expect("Parent not found");
                p_nu.calculated_offset()
            }
            None => 0,
        };

        let unit = NodeUnit {
            calculated_parent_offset,
            node,
        };

        self.nodes.insert(node_id, unit);
    }
    pub fn render(&self, cursor: &Cursor) -> String {
        // TODO - make nodes BTreeMap<NodeId,Vec<NodeUnit>>
        // NO! Do it from child to parent after all
        // this will allow scrolling to render a partial document. start with the cursor

        let mut buf = String::new();

        let Cursor {
            node_id, offset, ..
        } = cursor;

        let mut hopper = Vec::new();
        hopper.push(node_id.clone());

        while let Some(node_id) = hopper.pop() {
            let nu = self.nodes.get(&node_id).expect("Node not found");
            if let Some(parent) = &nu.node.parent {
                hopper.push(parent.clone());
            }
            nu.node.materialize(&mut buf);
        }

        buf
    }
}
