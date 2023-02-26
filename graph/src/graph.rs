use std::fmt;

use rand::{thread_rng, Rng};

/// Represents a graph with adjacency matrix- `graph.matrix`
///
/// # Examples
///
/// Graph can be created from adjacency matrix
/// ```rust
/// use graph::Graph;
///
/// let matrix = vec![vec![0,1,2], vec![1,0,3], vec![2,3,0]];
/// let g = Graph::new(matrix);
///
/// println!("{g}");
/// /*
///  * 0 1 2
///  * 1 0 3
///  * 2 3 0
/// */
/// ```
///
/// or Randomly with predetermined properties
/// ```rust
/// use graph::Graph;
///
/// let g = Graph::generate_random(10, 100);
/// ```
///
#[derive(Debug)]
pub struct Graph {
    pub matrix: Vec<Vec<usize>>,
}

impl Graph {
    /// Create a new `Graph` from adjacency matrix
    ///
    /// # Examples
    ///
    /// Graph can be created from adjacency matrix
    /// ```rust
    /// use graph::Graph;
    ///
    /// let matrix = vec![vec![0,1,2], vec![1,0,3], vec![2,3,0]];
    /// let g = Graph::new(matrix);
    ///
    /// println!("{g}");
    /// /*
    ///  * 0 1 2
    ///  * 1 0 3
    ///  * 2 3 0
    /// */
    /// ```
    pub fn new(matrix: Vec<Vec<usize>>) -> Self {
        Self { matrix }
    }

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
    /// let g = Graph::generate_random(10, 100);
    /// ```
    pub fn generate_random(city_count: usize, max_weight: usize) -> Self {
        let min_weight = max_weight / 2;
        let mut matrix = vec![vec![0; city_count]; city_count];
        let mut rng = thread_rng();

        for i in 0..city_count {
            for j in 0..city_count {
                if j < i {
                    matrix[i][j] = matrix[j][i];
                } else if i == j {
                    matrix[i][j] = 0;
                } else {
                    matrix[i][j] = rng.gen_range(min_weight..max_weight);
                }
            }
        }

        Self { matrix }
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
