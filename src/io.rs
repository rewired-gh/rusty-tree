use crate::parse::*;
use crate::tree::*;
use std::{fs::File, io::Read, *};

pub const DBG_PATH_TO_FILE: &str = "./assets/debug.yart";

pub fn get_tree_from_file(path: &str) -> Result<Tree, Box<dyn error::Error>> {
    let mut file = File::open(DBG_PATH_TO_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

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
