use core::fmt;

use rand::{thread_rng, Rng};

pub struct Graph {
    pub min_weight: usize,
    pub max_weight: usize,
    pub city_count: usize,
    pub matrix: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates a new graph, represented by adjacency matrix with following properties
    /// - complete = every vertex has direct edge to every other vertex
    /// - weighted = every edge has its weight
    /// - unordered = d(a,b) = d(b,a)
    /// - random = every edge has randomly generated weight
    /// - triangular inequity = d(a,b) <= d(a,c) + d(b,c)
    ///
    /// # Examples 
    /// ```rust
    /// use graph::Graph;
    ///
    /// let g = Graph::new(10, 100);
    /// ```
    pub fn new(city_count: usize, max_weight: usize) -> Self {
        let min_weight = max_weight/2;
        let mut matrix = vec![vec![0; city_count]; city_count];
        let mut rng = thread_rng();

        for i in 0..city_count {
            for j in 0..city_count {
                if j<i {
                    matrix[i][j] = matrix[j][i];
                } else if i == j {
                    matrix[i][j] = 0;
                } else {
                    matrix[i][j] = rng.gen_range(min_weight..max_weight);
                } 
            }
        }

        Self {
            min_weight,
            max_weight,
            city_count,
            matrix,
        }
    }

    pub fn find_shortest_path_brute_force(&self, start: usize) -> usize{
        let mut path = Vec::new();
        let mut paths = Vec::new();
        let mut seen = vec![false; self.city_count];

        self.walk(start, &mut path, &mut paths, &mut seen);

        let mut min = usize::MAX;
        for valid_path in paths {
            let mut sum = 0;
            for v in 0..self.city_count {
                sum += self.matrix[valid_path[v]][valid_path[v+1]];
            }
            println!("{sum}");
            if sum < min {
                min = sum;
            }
        }
        println!("{min}");

        min
    }

    fn walk(&self, current: usize, path: &mut Vec<usize>, paths: &mut Vec<Vec<usize>>, seen: &mut Vec<bool>) -> bool {
        // base case
        if path.len() == self.city_count {
            path.push(path[0]);
            paths.push(path.clone());
            path.pop();
            return true;
        }
        if seen[current] {
            return false;
        }

        // recurse
        path.push(current);
        seen[current] = true;

        for next_vertex in 0..self.city_count {
            if self.matrix[current][next_vertex] == 0 {
                continue;
            }
            if self.walk(next_vertex, path, paths, seen) {
                break;
            }
        }

        path.pop();
        seen[current] = false;

        false
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create_random_graph() {
        let g1 = Graph::new(10, 100);
        let g2 = Graph::new(10, 100);
        
        assert_ne!(g1.matrix, g2.matrix);
    }

    #[test]
    fn check_triangular_inequity() {
        let g = Graph::new(10, 100);
        assert_eq!(g.matrix[2][2],0);
        assert_eq!(g.matrix[2][1],g.matrix[1][2]);
        assert!(g.matrix[0][1]<=(g.matrix[0][2]+g.matrix[2][1]));
    }

    #[test]
    fn test() {
        let g = Graph::new(10, 100);
        g.find_shortest_path_brute_force(0);
    }
}
