mod approx_graph;

use approx_graph::solve_tsp;
use clap::Parser;
use graph::GraphFs;

#[derive(Parser, Debug)]
struct Args {
    /// path to source file with graphs
    source: String,
}

fn main() -> std::io::Result<()> {

    let mut graphs = GraphFs::new();
    graphs.load_from_file("../in.graph")?;

    for g in graphs {

        let (path, len) = solve_tsp(&g, 0);

        println!("path: {:?}", path);
        println!("length: {}", len);
    }

    Ok(())
}
