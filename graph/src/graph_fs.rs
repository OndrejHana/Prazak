use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::graph::*;

/// Wrapper around `graph::Graph`, responsible for loading and writing graphs to file system
///
/// `GraphFs` can hold multiple graphs. Loop trough them with iterator
///
/// # Examples
///
/// Create a new empty `GraphFs` and add some graphs to it
/// ```rust
/// use graph::*;
///
/// let mut graphs = GraphFs::new();
///
/// graphs.add(Graph::generate_random(5, 100));
/// graphs.add(Graph::generate_random(5, 100));
/// ```
///
/// Store all graphs in `GraphFs` instance into a specified file
/// ```rust
/// use graph::*;
///
/// let mut graphs = GraphFs::new();
///
/// graphs.add(Graph::generate_random(5, 100));
/// graphs.add(Graph::generate_random(5, 100));
/// graphs.add(Graph::generate_random(5, 100));
///
/// graphs.store_all_to_file("temp.graph").unwrap();
///
/// # std::fs::remove_file("./temp.graph").unwrap();
/// ```
///
/// Load graphs from a file
/// ```rust, ignore
/// use graph::*;
///
/// # let mut temp_graphs = GraphFs::new();
/// # temp_graphs.add(Graph::generate_random(5, 100));
/// # temp_graphs.add(Graph::generate_random(5, 100));
/// # temp_graphs.add(Graph::generate_random(5, 100));
/// #
/// # temp_graphs.store_all_to_file("temp1.graph").unwrap();
///
/// let mut graphs = GraphFs::new();
///
/// graphs.load_from_file("temp1.graph").expect("file not found");
///
/// for graph in graphs {
///     println!("{graph}");
/// }
///
/// # std::fs::remove_file("./temp1.graph").unwrap();
/// ```
pub struct GraphFs {
    graphs: VecDeque<Graph>,
}

impl GraphFs {
    /// Create a new, empty `GraphFs`
    ///
    /// # Examples
    ///
    /// Create a new empty `GraphFs` and add some graphs to it
    /// ```rust
    /// use graph::*;
    ///
    /// let mut graphs = GraphFs::new();
    ///
    /// graphs.add(Graph::generate_random(5, 100));
    /// graphs.add(Graph::generate_random(5, 100));
    /// ```
    pub fn new() -> Self {
        Self {
            graphs: VecDeque::new(),
        }
    }

    /// Add `Graph` to existing GraphFs
    ///
    /// # Examples
    ///
    /// Create a new empty `GraphFs` and add some graphs to it
    /// ```rust
    /// use graph::*;
    ///
    /// let mut graphs = GraphFs::new();
    ///
    /// graphs.add(Graph::generate_random(5, 100));
    /// graphs.add(Graph::generate_random(5, 100));
    /// ```
    pub fn add(&mut self, g: Graph) {
        self.graphs.push_back(g);
    }

    /// Store all graphs in `GraphFs` instance into a specified file
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use graph::*;
    ///
    /// let mut graphs = GraphFs::new();
    ///
    /// graphs.add(Graph::generate_random(5, 100));
    /// graphs.add(Graph::generate_random(5, 100));
    /// graphs.add(Graph::generate_random(5, 100));
    ///
    /// graphs.store_all_to_file("temp2.graph").unwrap();
    ///
    /// # std::fs::remove_file("./temp2.graph").unwrap();
    /// ```
    pub fn store_all_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = fs::File::create(path)?;

        for graph in &self.graphs {
            for row in &graph.matrix {
                writeln!(file, "{:?}", row)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }

    /// Load graphs from a file
    ///
    /// # Examples
    /// ```rust,ignore
    /// use graph::*;
    ///
    /// # let mut temp_graphs = GraphFs::new();
    /// # temp_graphs.add(Graph::generate_random(5, 100));
    /// # temp_graphs.add(Graph::generate_random(5, 100));
    /// # temp_graphs.add(Graph::generate_random(5, 100));
    /// #
    /// # temp_graphs.store_all_to_file("temp3.graph").unwrap();
    ///
    ///
    /// let mut graphs = GraphFs::new();
    ///
    /// graphs.load_from_file("temp3.graph").expect("file not found");
    ///
    /// for graph in graphs {
    ///     println!("{graph}");
    /// }
    /// # std::fs::remove_file("./temp3.graph").unwrap();
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let file = fs::File::open(path)?;
        let buf_reader = BufReader::new(file);

        let mut matrix: Vec<Vec<usize>> = Vec::new();
        for data in buf_reader.lines() {
            let data = data?;

            if data.is_empty() {
                self.add(Graph::new(matrix.clone()));
                matrix = Vec::new();
                continue;
            }

            let row: Vec<usize> = data[1..data.len() - 1]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();

            matrix.push(row);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.graphs.len()
    }
}

impl Iterator for GraphFs {
    type Item = Graph;

    fn next(&mut self) -> Option<Self::Item> {
        self.graphs.pop_front()
    }
}

impl Default for GraphFs {
    fn default() -> Self {
        Self::new()
    }
}
