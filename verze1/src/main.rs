use std::time;
use graph::Graph;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Number of cities(vertices) in a graph(it is recommended not to go above at most 20)
    city_count: usize,

    /// the upper bound value for weight of nodes. Lower bound will be calculated to preserve triangular inequity 
    max_line_weight: usize,
}

fn main() {
    let args = Args::parse();
    let g = Graph::new(args.city_count ,args.max_line_weight);
    let now = time::Instant::now();
    let (shortest_path_weight, path) =  g.find_shortest_path_brute_force(0);
    let elapsed = now.elapsed();

    println!("Graph:\n{}", g);
    println!("Shortest path: {:?}", path);
    println!("with weight of: {}", shortest_path_weight);
    println!("Elapsed: {}ms", elapsed.as_millis());
}
