mod error;
mod approx_graph;

use crate::error::*;

use approx_graph::ApproxGraph;
use graph::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    source: String,
    destination: String
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let mut graphs = GraphFs::new();
    graphs.load_from_file(&args.source)?;

    for g in graphs {
        let approx_g = ApproxGraph::new(g);

        let (path, len) = approx_g.solve_tsp(0);

        println!("path: {:?}", path);
        println!("length: {}", len);
    }

    Ok(())
}
