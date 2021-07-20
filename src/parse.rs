use crate::tree::*;
use std::{error::Error, mem::swap, *};

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

pub fn set_tree(tree: &mut Tree, lines: &Vec<&str>) -> Result<(), Box<dyn error::Error>> {
    if lines.is_empty() {
        return Err("Could not found any effective line in the file.".into());
    }

    let mut node_arrays_last = Vec::new();
    let mut node_arrays = Vec::new();

    set_node_arrays(tree, &mut node_arrays_last, &lines[0])?;

    if node_arrays_last.len() == 1 {
        if node_arrays_last[0].end - node_arrays_last[0].start != 1 {
            return Err("More than one possible root nodes were found.".into());
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
                            return Err("An unaligned group was found.".into());
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
        return Err("More than one possible root nodes were found.".into());
    }

    Ok(())
}

fn set_node_arrays(
    tree: &mut Tree,
    node_arrays: &mut Vec<NodeArray>,
    line: &str,
) -> Result<(), Box<dyn error::Error>> {
    let mut position = 0;
    let mut is_inside = false;
    let mut current_array = NodeArray::new();
    let mut current_name = String::new();
    for char in line.chars() {
        match char {
            '[' => {
                if is_inside {
                    return Err("An unclosed left bracket was found.".into());
                } else {
                    current_array = NodeArray::from_details(position, tree.nodes.len());
                    is_inside = true;
                }
            }
            ']' => {
                if !is_inside {
                    return Err("An unclosed right bracket was found.".into());
                } else if !current_name.is_empty() {
                    tree.nodes.push(Node::from_details(
                        position - current_name.len(),
                        current_name,
                    ));
                    current_name = String::new();
                }
                current_array.end = tree.nodes.len();
                if current_array.end == current_array.start {
                    return Err("An empty group was found.".into());
                }
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
                    return Err("A node was found in the wild.".into());
                }
            }
        };
        position += 1;
    }
    if is_inside {
        Err("An unclosed left bracket was found.".into())
    } else {
        Ok(())
    }
}
