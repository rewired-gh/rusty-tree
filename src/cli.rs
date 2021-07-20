use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Read a tree from the file. (Default file path: `./tree.yart`)
    Read,
}

#[derive(StructOpt)]
#[structopt(name = "Rusty Tree", about = "A command-line YART tree processor.")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different input tree file.
    #[structopt(parse(from_os_str), short, long)]
    pub input_file: Option<PathBuf>,
}
