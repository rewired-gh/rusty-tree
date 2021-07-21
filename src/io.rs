use crate::parse::*;
use crate::tree::*;
use anyhow::Result;
use std::path::PathBuf;
use std::*;

pub fn get_tree_from_file(path: PathBuf) -> Result<Tree> {
    let content = fs::read_to_string(path)?;

    let lines = clean_lines(content.lines().collect());
    let mut new_tree = Tree::new();
    set_tree(&mut new_tree, &lines)?;

    Ok(new_tree)
}

fn clean_lines(lines: Vec<&str>) -> Vec<&str> {
    let mut modified_lines: Vec<&str> = Vec::new();
    for mut line in lines {
        match line.split_once("#") {
            Some(tuple) => {
                line = tuple.0;
                if !line.trim().is_empty() {
                    modified_lines.push(line.trim_end());
                }
            }
            None => {
                if !line.trim().is_empty() {
                    modified_lines.push(line.trim_end());
                }
            }
        }
    }
    modified_lines
}
