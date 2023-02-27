mod approx_graph;
use crate::approx_graph::solve_tsp;

use clap::Parser;
use graph::GraphFs;
use std::time;

#[derive(Debug, Parser)]
struct Args {
    /// Path to a .graph file. Its graphs will be taken as input
    source: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let mut graphs = GraphFs::new();

    graphs.load_from_file(args.source)?;

    for graph in graphs {
        let now = time::Instant::now();
        let (path, cost) = solve_tsp(&graph);
        let elapsed = now.elapsed();

        println!("Graph:\n{}", &graph);
        println!("Shortest path: {:?}", path);
        println!("with weight of: {}", cost);
        println!("Elapsed: {}ms", elapsed.as_millis());
    }

    Ok(())
}
