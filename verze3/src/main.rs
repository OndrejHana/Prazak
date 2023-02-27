mod approx_graph;

use std::time;

use approx_graph::*;
use clap::Parser;
use graph::GraphFs;

#[derive(Parser, Debug)]
struct Args {
    /// path to source file with graphs
    source: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let mut graphs = GraphFs::new();
    graphs.load_from_file(args.source)?;

    for g in graphs {
        let now = time::Instant::now();

        let (path, len) = solve_tsp(&g);

        let elapsed = now.elapsed();

        println!("path: {:?}", path);
        println!("length: {}", len);
        println!("Elapsed: {}ms", elapsed.as_millis());
    }

    Ok(())
}
