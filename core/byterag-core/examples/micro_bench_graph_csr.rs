use std::time::Instant;

struct CsrGraph {
    row_offsets: Vec<u32>,
    col_indices: Vec<u32>,
}

impl CsrGraph {
    pub fn new_random(num_nodes: u32, edges_per_node: u32) -> Self {
        let mut row_offsets = Vec::with_capacity((num_nodes + 1) as usize);
        let mut col_indices = Vec::with_capacity((num_nodes * edges_per_node) as usize);

        let mut offset = 0;
        row_offsets.push(offset);

        for u in 0..num_nodes {
            for i in 1..=edges_per_node {
                let v = (u + i * 7) % num_nodes;
                col_indices.push(v);
                offset += 1;
            }
            row_offsets.push(offset);
        }

        Self { row_offsets, col_indices }
    }

    pub fn multi_hop_bfs(&self, start_node: u32, max_depth: usize) -> usize {
        let mut visited = vec![false; self.row_offsets.len() - 1];
        let mut current_frontier = vec![start_node];
        visited[start_node as usize] = true;
        let mut total_visited = 1;

        for _ in 0..max_depth {
            let mut next_frontier = Vec::new();
            for &u in &current_frontier {
                let start_idx = self.row_offsets[u as usize] as usize;
                let end_idx = self.row_offsets[(u + 1) as usize] as usize;
                for &v in &self.col_indices[start_idx..end_idx] {
                    if !visited[v as usize] {
                        visited[v as usize] = true;
                        next_frontier.push(v);
                        total_visited += 1;
                    }
                }
            }
            if next_frontier.is_empty() {
                break;
            }
            current_frontier = next_frontier;
        }

        total_visited
    }
}

fn main() {
    println!("============================================================");
    println!("   DBX Graph CSR Micro-Benchmark (100K Nodes, 500K Edges)  ");
    println!("============================================================");

    let num_nodes = 100_000;
    let edges_per_node = 5;
    println!("[1] Building CSR In-Memory Graph (100,000 Nodes, 500,000 Edges)...");
    let graph = CsrGraph::new_random(num_nodes, edges_per_node);

    // Benchmark 1-hop, 2-hop, 3-hop traversal
    for depth in 1..=3 {
        let start = Instant::now();
        let iters: usize = 1000;
        let mut sum_nodes: usize = 0;
        for i in 0..iters {
            sum_nodes += graph.multi_hop_bfs(((i * 97) % (num_nodes as usize)) as u32, depth);
        }
        let dur = start.elapsed();
        let avg_dur = dur.as_secs_f64() * 1000.0 / (iters as f64);
        println!("  >> {}-Hop BFS Traversal ({} iters): Avg {:.3} µs / query (visited total: {})", 
            depth, iters, avg_dur * 1000.0, sum_nodes / iters);
    }
    println!("============================================================");
}
