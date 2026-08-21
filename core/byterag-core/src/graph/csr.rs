use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Subgraph {
    pub nodes: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

/// Compressed Sparse Row (CSR) Graph
pub struct CsrGraph {
    pub num_nodes: usize,
    pub row_offsets: Vec<u32>,
    pub col_indices: Vec<u32>,
    pub node_map: HashMap<String, u32>,
    pub id_to_name: Vec<String>,
}

impl CsrGraph {
    pub fn new() -> Self {
        Self {
            num_nodes: 0,
            row_offsets: vec![0],
            col_indices: Vec::new(),
            node_map: HashMap::new(),
            id_to_name: Vec::new(),
        }
    }

    /// Build CSR graph from adjacency list: Vec<(src, dst)>
    pub fn from_edges(edges: &[(String, String)]) -> Self {
        let mut node_map = HashMap::new();
        let mut id_to_name = Vec::new();

        let mut get_or_insert_node = |name: &str| -> u32 {
            if let Some(&id) = node_map.get(name) {
                id
            } else {
                let id = id_to_name.len() as u32;
                node_map.insert(name.to_string(), id);
                id_to_name.push(name.to_string());
                id
            }
        };

        // Convert string edges to ID pairs
        let mut id_edges: Vec<(u32, u32)> = Vec::with_capacity(edges.len());
        for (src, dst) in edges {
            let u = get_or_insert_node(src);
            let v = get_or_insert_node(dst);
            id_edges.push((u, v));
        }

        let num_nodes = id_to_name.len();
        // Sort edges by source node
        id_edges.sort_by_key(|e| (e.0, e.1));

        let mut row_offsets = Vec::with_capacity(num_nodes + 1);
        let mut col_indices = Vec::with_capacity(id_edges.len());

        let mut current_node = 0;
        row_offsets.push(0);

        for (u, v) in id_edges {
            while current_node < u {
                row_offsets.push(col_indices.len() as u32);
                current_node += 1;
            }
            col_indices.push(v);
        }

        while row_offsets.len() <= num_nodes {
            row_offsets.push(col_indices.len() as u32);
        }

        Self {
            num_nodes,
            row_offsets,
            col_indices,
            node_map,
            id_to_name,
        }
    }

    pub fn get_node_id(&self, name: &str) -> Option<u32> {
        self.node_map.get(name).copied()
    }

    pub fn get_node_name(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(id as usize).map(|s| s.as_str())
    }

    pub fn neighbors(&self, u: u32) -> &[u32] {
        if (u as usize) >= self.num_nodes {
            return &[];
        }
        let start = self.row_offsets[u as usize] as usize;
        let end = self.row_offsets[(u + 1) as usize] as usize;
        &self.col_indices[start..end]
    }

    /// Breadth-First Multi-hop Traversal
    pub fn multi_hop_traversal(&self, start_name: &str, max_depth: usize) -> Option<Subgraph> {
        let start_id = self.get_node_id(start_name)?;
        let mut visited = vec![false; self.num_nodes];
        let mut queue = VecDeque::new();
        let mut result_nodes = Vec::new();
        let mut result_edges = Vec::new();

        visited[start_id as usize] = true;
        queue.push_back((start_id, 0));
        result_nodes.push(start_id);

        while let Some((u, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            for &v in self.neighbors(u) {
                result_edges.push((u, v));
                if !visited[v as usize] {
                    visited[v as usize] = true;
                    result_nodes.push(v);
                    queue.push_back((v, depth + 1));
                }
            }
        }

        Some(Subgraph {
            nodes: result_nodes,
            edges: result_edges,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csr_graph_traversal() {
        let edges = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
            ("B".to_string(), "D".to_string()),
            ("C".to_string(), "E".to_string()),
            ("D".to_string(), "F".to_string()),
        ];

        let graph = CsrGraph::from_edges(&edges);
        assert_eq!(graph.num_nodes, 6);

        let sub = graph.multi_hop_traversal("A", 2).expect("Node A exists");
        // A -> (B, C) -> (D, E) => Nodes: A, B, C, D, E
        assert_eq!(sub.nodes.len(), 5);
        assert_eq!(sub.edges.len(), 4);
    }
}
