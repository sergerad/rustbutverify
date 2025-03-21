use crate::parse::parse_adjacency_list;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

/// List of directed edges from one node to a set of others.
type AdjacencyList<T> = HashMap<T, Vec<T>>;

/// A directed graph that is capable of detecting cycles.
#[derive(Debug, Default)]
pub struct Graph<T> {
    adjacency_list: AdjacencyList<T>,
}

impl<T: Eq + Hash + Clone> Graph<T> {
    /// Finds cycles in the directed graph.
    ///
    /// Returns the first cycle detected.
    pub fn find_cycle(&self) -> Option<Vec<T>> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for node in self.adjacency_list.keys() {
            if self.dfs(node, &mut visited, &mut stack) {
                return Some(stack);
            }
        }
        None
    }

    /// Depth-first search used to find cycles in the graph.
    fn dfs(&self, node: &T, visited: &mut HashSet<T>, stack: &mut Vec<T>) -> bool {
        if stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visited.insert(node.clone());
        stack.push(node.clone());
        if let Some(neighbors) = self.adjacency_list.get(node) {
            for n in neighbors {
                if self.dfs(n, visited, stack) {
                    return true;
                }
            }
        }

        stack.pop();
        false
    }
}

impl<T, E> TryFrom<&str> for Graph<T>
where
    T: Eq + Hash + Clone + FromStr<Err = E>,
    E: Debug,
{
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, adjacency_list) = parse_adjacency_list(value)
            .map_err(|e| anyhow::anyhow!(format!("failed to parse: {}", e.to_string())))?;
        Ok(Self { adjacency_list })
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn parse_cycle() {
        let s = "A\tB\nB\tC\nC\tA\n";
        let g = Graph::<String>::try_from(s).unwrap();
        assert!(g.find_cycle().is_some());
    }

    #[test]
    fn parse_cycle_output() {
        let s = "A\tB\nB\tA\n";
        let g = Graph::<String>::try_from(s).unwrap();
        assert!(g.find_cycle().is_some());
        assert_eq!(g.find_cycle().unwrap().len(), 2);
        // Output order depends on a loop over hash set, so cannot assume ordering.
        assert!(g.find_cycle().unwrap().contains(&"A".to_string()));
        assert!(g.find_cycle().unwrap().contains(&"B".to_string()));
    }

    #[test]
    fn parse_no_cycle() {
        let s = "A\tB\nB\tC\nC\tD\n";
        let g = Graph::<String>::try_from(s).unwrap();
        assert!(g.find_cycle().is_none());
    }
}
