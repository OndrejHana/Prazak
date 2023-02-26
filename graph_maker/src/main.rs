use graph::{Graph, GraphFs};
fn main() -> std::io::Result<()> {
    let mut graphs = GraphFs::new();

    graphs.add(Graph::generate_random(10, 20));
    graphs.add(Graph::generate_random(10, 20));
    graphs.add(Graph::generate_random(10, 20));
    graphs.add(Graph::generate_random(10, 20));
    graphs.add(Graph::generate_random(10, 20));

    graphs.store_all_to_file("../in.graph")?;

    return Ok(());
}
