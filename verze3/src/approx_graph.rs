use std::{collections::BinaryHeap, cmp::{Reverse, Ordering}};

use graph::Graph;

#[derive(Eq, Debug)]
struct Edge {
    u: usize,
    v: usize,
    weight: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.weight.cmp(&other.weight);
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        return self.weight == other.weight;
    }
}


pub struct ApproxGraph {
    graph: Graph,
}

impl ApproxGraph {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph
        }
    }
    pub fn solve_tsp(&self, start: usize) -> (Vec<usize>, usize) {
        let sorted_edges = self.get_mst(start);

        todo!()
    }

    fn get_mst(&self, start: usize) -> Vec<usize> {
        let mst = Vec::new();

        let sorted_edges = self.get_sorted_edge_list();

        println!("{sorted_edges:?}");

        return mst;
    }


    fn get_sorted_edge_list(&self) -> BinaryHeap<Reverse<Edge>> {
        // using min binary heap to instantly sort for Kruskals algorithm
        let mut heap = BinaryHeap::new();

        for i in 0..self.graph.matrix.len() {
            let row = &self.graph.matrix[i];

            for j in 0..i {
                if row[j] == 0 {
                    continue;
                }

                heap.push(Reverse(Edge {
                    u: i,
                    v: j,
                    weight: row[j]
                }));
            }

        }
        return heap;
    }
}
