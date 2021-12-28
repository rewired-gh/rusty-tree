use crate::tree::*;
use anyhow::{bail, Result};
use std::{mem::swap, *};

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

pub fn set_tree(tree: &mut Tree, lines: &Vec<&str>) -> Result<()> {
    if lines.is_empty() {
        bail!("Could not find any effective line in the file.");
    }

    let mut node_arrays_last = Vec::new();
    let mut node_arrays = Vec::new();

    set_node_arrays(tree, &mut node_arrays_last, &lines[0])?;

    if node_arrays_last.len() == 1 {
        if node_arrays_last[0].end - node_arrays_last[0].start != 1 {
            bail!("More than one possible root nodes were found.");
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
                            bail!("An unaligned group was found.");
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
        bail!("More than one possible root nodes were found.");
    }

    Ok(())
}

fn set_node_arrays(tree: &mut Tree, node_arrays: &mut Vec<NodeArray>, line: &str) -> Result<()> {
    let mut position = 0;
    let mut is_inside = false;
    let mut current_array = NodeArray::new();
    let mut current_name = String::new();
    for char in line.chars() {
        match char {
            '[' => {
                if is_inside {
                    bail!("An unclosed left bracket was found.");
                } else {
                    current_array = NodeArray::from_details(position, tree.nodes.len());
                    is_inside = true;
                }
            }
            ']' => {
                if !is_inside {
                    bail!("An unclosed right bracket was found.");
                } else if !current_name.is_empty() {
                    tree.nodes.push(Node::from_details(
                        position - current_name.len(),
                        current_name,
                    ));
                    current_name = String::new();
                }
                current_array.end = tree.nodes.len();
                if current_array.end == current_array.start {
                    bail!("An empty group was found.");
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
                    bail!("A node was found in the wild.");
                }
            }
        };
        position += 1;
    }
    if is_inside {
        bail!("An unclosed left bracket was found.")
    }
    Ok(())
}
