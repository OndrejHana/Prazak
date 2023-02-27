use clap::Parser;
use graph::{Graph, GraphFs};

#[derive(Debug, Parser)]
struct Args {
    /// Number of graphs generated
    graph_count: usize,

    /// Number of vertices in each graph
    vertex_count: usize,

    /// Path for the output file
    destination: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let mut graphs = GraphFs::new();

    for _ in 0..args.graph_count {
        graphs.add(Graph::generate_random(args.vertex_count, 100));
    }

    graphs.store_all_to_file(args.destination)?;

    return Ok(());
}
