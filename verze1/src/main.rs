use verze1::solve_tsp_brute_force;

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
        let (shortest_path_weight, path) = solve_tsp_brute_force(&graph, 0);
        let elapsed = now.elapsed();

        println!("Graph:\n{}", &graph);
        println!("Shortest path: {:?}", path);
        println!("with weight of: {}", shortest_path_weight);
        println!("Elapsed: {}ms", elapsed.as_millis());
    }

    Ok(())
}
