use std::{collections::HashMap, sync::mpsc, thread};

use graph::Graph;
use num_cpus;

#[derive(Debug)]
struct SubGraph {
    seq_num: usize,
    graph: Graph,
    vertex_map: HashMap<usize, usize>,
    path: Vec<usize>,
}

pub fn solve_tsp(graph: &Graph) -> (Vec<usize>, usize) {
    let k = num_cpus::get();
    let parts = partition_graph(graph, k);

    let mut handles = Vec::new();
    let (tx, rx) = mpsc::channel();
    for mut subgraph in parts {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            let subgraph_path = nearest_neighbor(&subgraph.graph, 0);

            for v in subgraph_path {
                subgraph.path.push(*subgraph.vertex_map.get(&v).unwrap());
            }

            if let Err(_) = tx.send(subgraph) {
                return;
            };
        });

        handles.push(handle);
    }

    let mut subgraph_count = 0;
    let mut path = vec![0; graph.matrix.len() + 1];

    while subgraph_count < k {
        let subgraph = rx.recv().unwrap();

        for i in 0..subgraph.path.len() {
            let path_index = subgraph.seq_num * subgraph.path.len() + i;
            path[path_index] = subgraph.path[i];
        }

        subgraph_count += 1;
    }

    let path_cost = get_cost(graph, &path);

    return (path, path_cost);
}

fn partition_graph(graph: &Graph, k: usize) -> Vec<SubGraph> {
    let mut partitions = Vec::new();

    let partition_size = graph.matrix.len() / k;

    for i in 0..k {
        let mut partition = Vec::new();
        let mut v_map: HashMap<usize, usize> = HashMap::new();
        for j in i * partition_size..(i + 1) * partition_size {
            partition.push(graph.matrix[j][i * partition_size..(i + 1) * partition_size].to_vec());
            v_map.insert(partition.len() - 1, j);
        }

        let subgraph = SubGraph {
            seq_num: i,
            graph: Graph::new(partition),
            vertex_map: v_map,
            path: Vec::with_capacity(partition_size),
        };
        partitions.push(subgraph);
    }

    return partitions;
}

fn nearest_neighbor(graph: &Graph, start: usize) -> Vec<usize> {
    let mut current = start;
    let mut path = Vec::new();
    let mut seen = vec![false; graph.matrix.len()];

    path.push(current);
    seen[current] = true;

    while path.len() < graph.matrix.len() {
        let mut min = usize::MAX;
        let mut closest_neighbor = None;
        for (neighbor, weight) in graph.matrix[current].iter().enumerate() {
            if *weight == 0 {
                continue;
            }

            if seen[neighbor] {
                continue;
            }

            if *weight < min {
                closest_neighbor = Some(neighbor);
                min = *weight;
            }
        }

        if let Some(next_v) = closest_neighbor {
            path.push(next_v);
            seen[next_v] = true;

            current = next_v;
        }
    }

    return path;
}

fn get_cost(graph: &Graph, path: &Vec<usize>) -> usize {
    let mut cost = 0;
    for i in 0..path.len() - 1 {
        cost += graph.matrix[path[i]][path[i + 1]];
    }
    return cost;
}
