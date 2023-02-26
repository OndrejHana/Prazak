use graph::Graph;

pub fn do_something(graph: Graph, start: usize) -> (Vec<usize>, usize) {
    let mut current = start;
    let mut path = Vec::new();
    let mut seen = vec![false; graph.matrix.len()];

    path.push(current);
    seen[current] = true;

    while path.len() < graph.matrix.len() {
        let mut min = usize::MAX;
        let mut next_vertex = None;

        for neighbor in 0..graph.matrix.len() {
            if seen[neighbor] {
                continue;
            }

            if graph.matrix[current][neighbor] < min {
                min = graph.matrix[current][neighbor];
                next_vertex = Some(neighbor);
            }
        }

        if let Some(next_vertex) = next_vertex {
            path.push(next_vertex);
            seen[next_vertex] = true;
            current = next_vertex;
        }
    }

    let sum = path.iter().sum();
    return (path, sum);
}

pub fn solve_tsp(graph: &Graph, start: usize) -> (Vec<usize>, usize){
    let mut current = start;
    let mut path = Vec::new();
    let mut seen = vec![false; graph.matrix.len()];

    while path.len() < graph.matrix.len() {

        let mut min = usize::MAX;
        let mut closest_neighbor = None;
        for (i, weight) in graph.matrix[current].iter().enumerate() {
            let weight = weight.clone();
            if weight == 0 {
                continue;
            }

            if weight < min {
                closest_neighbor = Some(i);
                min = weight;
            }
        }

        if let Some(next_v) = closest_neighbor {
            path.push(current);
            seen[current] = true;

            current = next_v;
        }
    }

    path.push(start);

    let mut cost = 0;
    for i in 0..path.len()-1 {
        let curr = path[i];
        let next = path[i+1];

        cost += graph.matrix[curr][next];
    }

    return (path, cost);
}
