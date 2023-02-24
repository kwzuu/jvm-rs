use crate::Runtime;
use clap::Parser;

/*
/// Cli parser
#[inline]
fn cli() -> Vec<Arg> {
    let mut args = Vec::new();
    let mut iter = std::env::args().skip(1).peekable();
    while let Some(it) = iter.next() {
        let lc = it.to_lowercase();
        if lc.starts_with("xms") {

        }
    }
    todo!()
}

fn parse_size() {
    
}

// all numbers in bytes
struct MemoryConfig {
    xmx: Option<usize>,
    xms: Option<usize>,
    xss: Option<usize>,
 /* TODO: And out of the ground made the Lord God to grow every tree that 
    is pleasant to the sight, and good for food; the tree of life also
    in the midst of the garden, and the tree of knowledge of good and evil. */
    xmn: Option<usize>,
    xgc: Option<usize>, 
}

enum Executable {
    Class(String),
    Jar(String),
}

struct Args {
    memory_config: MemoryConfig,
    exec: Executable,
    dry: bool,
    program_args: Vec<String>,
    xx: Vec<String>,
    cp: Vec<String>,
}

enum Arg {
    // Loading params
    Module(String), // FIXME: We don't want to use this
    Jar(String), // TODO: support jar files
    AgentLib(String),
    AgentPath(String),
}
*/