mod error;
mod approx_graph;

use crate::error::*;

use approx_graph::solve_tsp;
use graph::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    source: String,
    destination: String
}

fn main() -> Result<()> {

    let mut graphs = GraphFs::new();
    graphs.add(Graph::generate_random(15, 100));

    for g in graphs {
        let (path, len) = solve_tsp(g, 0);

        println!("path: {:?}", path);
        println!("length: {}", len);
    }

    Ok(())
}
