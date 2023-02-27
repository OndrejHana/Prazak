use graph::Graph;

pub fn solve_tsp(graph: &Graph) -> (Vec<usize>, usize) {
    let path = nearest_neighbor(graph, 0);
    let cost = get_cost(graph, &path);

    return (path, cost);
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
