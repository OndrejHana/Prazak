use std::time;
use graph::Graph;

fn main() {
    let g = Graph::new(10, 100);
    let now = time::Instant::now();
    let (shortest_path_weight, path) =  g.find_shortest_path_brute_force(0);
    let elapsed = now.elapsed();

    println!("Graph:\n{}", g);
    println!("Shortest path: {:?}", path);
    println!("with weight of: {}", shortest_path_weight);
    println!("Elapsed: {}ms", elapsed.as_millis());
}
