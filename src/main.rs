use io::*;
use tree::Tree;

mod io;
mod tree;

fn main() {
    let tree = get_tree_from_file(DBG_PATH_TO_FILE).expect("msg");

    println!("\n=========================\n");
    dbg_ignite(&tree, 0);
}

fn dbg_ignite(tree: &Tree, index: usize) {
    dbg!(&tree.nodes[index]);
    if tree.nodes[index].children_index_end > 0 {
        for i in tree.nodes[index].children_index_start..tree.nodes[index].children_index_end {
            dbg_ignite(&tree, i);
        }
    }
}
