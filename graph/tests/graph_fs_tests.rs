use graph::Graph;
use graph::GraphFs;

#[test]
fn test_file_store() {
    let mut graphs = GraphFs::new();
    graphs.add(Graph::generate_random(5, 100));

    graphs.store_all_to_file("graphs").unwrap();
}

#[test]
fn test_store_multiple() {
    let mut graphs = GraphFs::new();
    graphs.add(Graph::generate_random(5, 100));
    graphs.add(Graph::generate_random(5, 100));
    graphs.add(Graph::generate_random(5, 100));

    graphs.store_all_to_file("graphs").unwrap();
    std::fs::remove_file("graphs").unwrap();
}

#[test]
fn test_store_load() {
    let mut graphs = GraphFs::new();
    let mut graphs2 = GraphFs::new();

    graphs.add(Graph::generate_random(5, 100));

    graphs.store_all_to_file("graphs").unwrap();

    graphs2.load_from_file("graphs").unwrap();

    let g1 = graphs.next().unwrap();
    let g2 = graphs2.next().unwrap();

    assert_eq!(g1.matrix, g2.matrix);
    std::fs::remove_file("graphs").unwrap();
}

#[test]
fn test_store_load_multiple() {
    let mut graphs = GraphFs::new();
    let mut graphs2 = GraphFs::new();

    graphs.add(Graph::generate_random(5, 100));
    graphs.add(Graph::generate_random(5, 100));
    graphs.add(Graph::generate_random(5, 100));

    graphs.store_all_to_file("graphs").unwrap();

    graphs2.load_from_file("graphs").unwrap();

    std::fs::remove_file("graphs").unwrap();

    loop {
        if let Some(g1) = graphs.next() {
            if let Some(g2) = graphs2.next() {
                assert_eq!(g1.matrix, g2.matrix);
            }
        } else {
            return;
        }
    }
}
