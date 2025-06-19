use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub struct Node {
    pub gene: String,
    pub node_type: String,
    pub frame: i32,
    pub position: i32,
}

impl Node {
    pub fn new(gene: &str, node_type: &str, frame: i32, position: i32) -> Self {
        Self {
            gene: gene.to_string(),
            node_type: node_type.to_string(),
            frame,
            position,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node({:?},{:?},{:?},{:?})",
            self.gene, self.node_type, self.frame, self.position
        )
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hash_key() == other.hash_key()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_key().hash(state);
    }
}

impl Node {
    fn hash_key(&self) -> String {
        // Hash based on the repr-like string used in Python
        format!("{:?},{:?},{:?},{:?}", self.gene, self.node_type, self.frame, self.position)
    }
}
