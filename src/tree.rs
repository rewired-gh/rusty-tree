use std::{mem::swap, *};

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

#[derive(Debug)]
pub struct ParseError {}

#[derive(Debug, Clone, Copy)]
struct NodeArray {
    position: usize,

    // [start, end)
    start: usize,
    end: usize,
}

impl NodeArray {
    fn new() -> NodeArray {
        NodeArray {
            position: 0,
            start: 0,
            end: 0,
        }
    }
    fn from_details(position: usize, start: usize) -> NodeArray {
        NodeArray {
            position,
            start,
            end: 0,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "A error occurred during parsing the file.")
    }
}

impl error::Error for ParseError {}

pub fn set_tree(tree: &mut Tree, lines: &Vec<&str>) -> Result<(), ParseError> {
    if lines.is_empty() {
        return Err(ParseError {});
    }

    let mut node_arrays_last = Vec::new();
    let mut node_arrays = Vec::new();

    set_node_arrays(tree, &mut node_arrays_last, &lines[0])?;

    if node_arrays_last.len() == 1 {
        if node_arrays_last[0].end - node_arrays_last[0].start != 1 {
            return Err(ParseError {});
        } else {
            tree.root_node_index = node_arrays_last[0].start;

            let mut depth = 1usize;

            while depth < lines.len() {
                // [last_start, last_end)
                let last_start = node_arrays_last.first().unwrap().start;
                let last_end = node_arrays_last.last().unwrap().end;

                let mut index = last_start;

                set_node_arrays(tree, &mut node_arrays, &lines[depth])?;

                for node_array in node_arrays.iter() {
                    loop {
                        if index >= last_end {
                            return Err(ParseError {});
                        } else if node_array.position == tree.nodes[index].position {
                            tree.nodes[index].children_index_start = node_array.start;
                            tree.nodes[index].children_index_end = node_array.end;
                            break;
                        } else {
                            index += 1;
                        }
                    }
                }

                swap(&mut node_arrays_last, &mut node_arrays);
                node_arrays.clear();
                depth += 1;
            }
        }
    } else {
        return Err(ParseError {});
    }

    Ok(())
}

fn set_node_arrays(
    tree: &mut Tree,
    node_arrays: &mut Vec<NodeArray>,
    line: &str,
) -> Result<(), ParseError> {
    let mut position = 0;
    let mut is_inside = false;
    let mut current_array = NodeArray::new();
    let mut current_name = String::new();
    for char in line.chars() {
        match char {
            '[' => {
                current_array = NodeArray::from_details(position, tree.nodes.len());
                is_inside = true;
            }
            ']' => {
                if !is_inside {
                    return Err(ParseError {});
                } else if !current_name.is_empty() {
                    tree.nodes.push(Node::from_details(
                        position - current_name.len(),
                        current_name,
                    ));
                    current_name = String::new();
                }
                current_array.end = tree.nodes.len();
                node_arrays.push(current_array);
                is_inside = false;
            }
            ' ' => {
                if is_inside {
                    if !current_name.is_empty() {
                        tree.nodes.push(Node::from_details(
                            position - current_name.len(),
                            current_name,
                        ));
                        current_name = String::new();
                    }
                }
            }
            _ => {
                if is_inside {
                    current_name.push(char);
                } else {
                    return Err(ParseError {});
                }
            }
        };
        position += 1;
    }
    Ok(())
}
