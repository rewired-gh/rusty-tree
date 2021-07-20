use io::*;
use tree::Tree;

mod io;
mod parse;
mod tree;

fn main() {
    let tree = get_tree_from_file(DBG_PATH_TO_FILE).expect("Parse Error");

    println!("\n========================================\n");
    dbg_ignite(&tree, 0, 0);
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
