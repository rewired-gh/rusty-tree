use anyhow::Result;
use cli::*;
use io::*;
use std::path::PathBuf;
use structopt::StructOpt;
use tree::Tree;

mod cli;
mod io;
mod parse;
mod tree;

const DEFAULT_FILE_PATH: &str = "./tree.yart";

fn main() -> Result<()> {
    let CommandLineArgs { action, input_file } = CommandLineArgs::from_args();
    let input_file = input_file.unwrap_or(PathBuf::from(DEFAULT_FILE_PATH));
    let tree;
    match action {
        Action::Read => tree = get_tree_from_file(input_file).unwrap(),
    };

    println!("\n========================================\n");
    dbg_ignite(&tree, tree.root_node_index, 0);

    Ok(())
}

fn dbg_ignite(tree: &Tree, index: usize, depth: usize) {
    print!("\x1b[0m-");
    for _i in 0..depth {
        print!("---");
    }
    print!("\x1b[0;93m[{}] \x1b[0;97m{}", depth, tree.nodes[index].name,);
    let children_count =
        tree.nodes[index].children_index_end - tree.nodes[index].children_index_start;
    if children_count > 0 {
        print!("  \x1b[0;36m({})", children_count);
    }
    println!();
    if tree.nodes[index].children_index_end > 0 {
        for i in tree.nodes[index].children_index_start..tree.nodes[index].children_index_end {
            dbg_ignite(&tree, i, depth + 1);
        }
    }
}
