use std::fs;

use graph::Graph;
use graph::GraphFs;

#[test]
fn test_file_store() {
    let mut graphs = GraphFs::new();
    let g1 = Graph::generate_random(3, 10);
    graphs.add(g1);

    graphs.store_all_to_file("./temp1.graph").unwrap();

    fs::File::open("./temp1.graph").unwrap();

    fs::remove_file("./temp1.graph").unwrap();
}

#[test]
fn test_file_store_load() {
    let mut graphs = GraphFs::new();
    let g1 = Graph::generate_random(3, 10);
    graphs.add(g1);

    graphs.store_all_to_file("./temp2.graph").unwrap();

    let mut graphs2 = GraphFs::new();

    graphs2.load_from_file("./temp2.graph").unwrap();

    for _ in 0..graphs.len() {
        if let Some(g1) = graphs.next() {
            if let Some(g2) = graphs2.next() {
                assert_eq!(g1.matrix, g2.matrix);
            }
        }
    }

    fs::remove_file("./temp2.graph").unwrap();
}

#[test]
fn test_file_store_load_multiple() {
    let mut graphs = GraphFs::new();

    graphs.add(Graph::generate_random(3, 10));
    graphs.add(Graph::generate_random(3, 10));
    graphs.add(Graph::generate_random(3, 10));
    graphs.add(Graph::generate_random(3, 10));
    graphs.add(Graph::generate_random(3, 10));

    graphs.store_all_to_file("./temp3.graph").unwrap();

    let mut graphs2 = GraphFs::new();

    graphs2.load_from_file("./temp3.graph").unwrap();

    for _ in 0..graphs.len() {
        if let Some(g1) = graphs.next() {
            if let Some(g2) = graphs2.next() {
                assert_eq!(g1.matrix, g2.matrix);
            }
        }
    }

    fs::remove_file("./temp3.graph").unwrap();
}
