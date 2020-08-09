use crate::cursor::Cursor;
use serde::Serialize;
use sha2::{Digest, Sha512Trunc256};

#[derive(Debug, Serialize)]
pub enum Action {
    Null,
    Insert {
        offset: u32,
        body: String,
    },
    #[allow(unused)]
    Delete {
        offset: u32,
    },
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeId(pub [u8; 32]);

pub struct Node {
    pub parent: Option<NodeId>,
    pub action: Action,
}

impl NodeId {
    pub fn hex4(&self) -> String {
        hex::encode(&self.0[0..2])
    }
}

static NULL: &'static [u8; 32] = &[0; 32];

impl Node {
    #[allow(unused)]
    pub fn new(parent: Option<NodeId>, action: Action) -> Self {
        Node {
            action,
            parent: parent,
        }
    }
    pub fn root() -> Node {
        Node {
            parent: None,
            action: Action::Null,
        }
    }
    pub fn insert(cursor: &Cursor, body: String) -> Self {
        Node {
            action: Action::Insert {
                offset: cursor.offset,
                body,
            },
            parent: Some(cursor.node_id.clone()),
        }
    }
    pub fn delete(cursor: &Cursor) -> Self {
        Node {
            action: Action::Delete {
                offset: cursor.offset,
            },
            parent: Some(cursor.node_id.clone()),
        }
    }
    pub fn parent(&self) -> Option<&NodeId> {
        self.parent.as_ref()
    }
    pub fn parent_hex4(&self) -> String {
        if let Some(p) = &self.parent {
            p.hex4()
        } else {
            "NA".to_string()
        }
    }
    pub fn offset(&self) -> u32 {
        use crate::node::Action::*;
        match &self.action {
            Null => 0,
            Insert { offset, .. } | Delete { offset, .. } => *offset,
        }
    }
    #[allow(unused)]
    pub fn node_id(&self) -> NodeId {
        let mut hasher = Sha512Trunc256::new();
        if let Some(parent) = self.parent() {
            hasher.update(parent.0);
        } else {
            hasher.update(NULL);
        }

        serde_json::to_writer(&mut hasher, &self.action);

        // read hash digest and consume hasher
        let result: [u8; 32] = hasher.finalize().into();

        NodeId(result)
    }
    pub fn materialize(&self, buf: &mut String) {
        match &self.action {
            crate::node::Action::Null => println!("{}: root", self.node_id().hex4()),
            crate::node::Action::Insert { offset, body } => {
                println!(
                    "{}: insert({}, {}) ({})",
                    self.node_id().hex4(),
                    offset,
                    body,
                    self.parent_hex4()
                );
                buf.insert_str(0, &body);
            }
            crate::node::Action::Delete { offset } => {
                println!(
                    "{}: delete({}) ({})",
                    self.node_id().hex4(),
                    offset,
                    self.parent_hex4()
                );
                buf.remove(*offset as usize);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Action, Node};

    #[test]
    fn test1() {
        let node0 = Node::root();
        let node1 = Node::new(
            Some(node0.node_id()),
            Action::Insert {
                offset: 0,
                body: "H".to_string(),
            },
        );

        let node2 = Node::new(
            Some(node1.node_id()),
            Action::Insert {
                offset: 0,
                body: "e".to_string(),
            },
        );

        let foo: &[u8; 32] = &(node2.node_id().0);
        assert_eq!(
            hex::encode(foo),
            "9d747eaf196585e24f421fc98ae818f77423a5d24d15cd5e869e43f2c25ee8d9"
        )
    }
}
