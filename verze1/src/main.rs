use verze1::solve_tsp_brute_force;

use std::time;
use graph::{Graph, GraphFs};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
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

/*
fn main() {
    let args = Args::parse();
    let g = Graph::generate_random(args.city_count ,args.max_line_weight);
    let now = time::Instant::now();
    let (shortest_path_weight, path) =  
    let elapsed = now.elapsed();

    println!("Graph:\n{}", g);
    println!("Shortest path: {:?}", path);
    println!("with weight of: {}", shortest_path_weight);
    println!("Elapsed: {}ms", elapsed.as_millis());
}
*/
