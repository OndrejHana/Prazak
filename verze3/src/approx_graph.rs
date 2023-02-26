use graph::Graph;

pub fn solve_tsp(graph: &Graph, start: usize) -> (Vec<usize>, usize){
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

    path.push(start);

    let mut cost = 0;
    for i in 0..path.len()-1 {
        let curr = path[i];
        let next = path[i+1];

        cost += graph.matrix[curr][next];
    }

    return (path, cost);
}
