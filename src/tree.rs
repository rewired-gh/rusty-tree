use std::*;

pub struct Tree {
    pub root_node_index: usize,
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root_node_index: 0,
            nodes: Vec::new(),
        }
    }
    pub fn root_node(&self) -> &Node {
        &self.nodes[self.root_node_index]
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: usize,
    pub name: String,

    //[children_index_start, children_index_end)
    pub children_index_start: usize,
    pub children_index_end: usize,
}

impl Node {
    pub fn new() -> Node {
        Node {
            position: 0,
            name: String::new(),
            children_index_start: 0,
            children_index_end: 0,
        }
    }
    pub fn from_details(position: usize, name: String) -> Node {
        Node {
            position,
            name,
            children_index_start: 0,
            children_index_end: 0,
        }
    }
}
