use graph::Graph;

pub fn solve_tsp_brute_force(graph: &Graph, start: usize) -> (usize, Vec<usize>) {
    let city_count = graph.matrix.len();
    let mut path = Vec::new();
    let mut shortest_path = Vec::new();
    let mut seen = vec![false; city_count];

    walk(&graph, start, &mut path, &mut shortest_path, &mut seen);

    let mut shortest_path_cost = 0;
    for i in 0..graph.matrix.len()-1 {
        shortest_path_cost += graph.matrix[shortest_path[i]][shortest_path[i+1]];
    }

    return (shortest_path_cost, shortest_path);
}

fn walk(graph: &Graph, current: usize, path: &mut Vec<usize>, shortest_path: &mut Vec<usize>, seen: &mut Vec<bool>) -> bool {
    let city_count = graph.matrix.len();

    // base case
    if path.len() == city_count {
        path.push(path[0]);

        if shortest_path.len() == 0 {
            *shortest_path = path.clone();
        }

        let mut shortest_path_cost = 0;
        let mut path_cost = 0;
        for i in 0..graph.matrix.len()-1 {
            shortest_path_cost += graph.matrix[shortest_path[i]][shortest_path[i+1]];
            path_cost += graph.matrix[path[i]][path[i+1]];
        }

        if path_cost < shortest_path_cost {
            *shortest_path = path.clone();
        }

        path.pop();
        return true;
    }

    if seen[current] {
        return false;
    }

    // recurse
    path.push(current);
    seen[current] = true;

    for next_vertex in 0..city_count {
        if graph.matrix[current][next_vertex] == 0 {
            continue;
        }
        if walk(graph, next_vertex, path, shortest_path, seen) {
            break;
        }
    }

    path.pop();
    seen[current] = false;

    false
}
