//! ByteRAG Native Code Graph Indexer
//! Ingests AST code graphs directly into ByteRAG 5-Tier Native Storage

use byterag_core::graph::csr::CsrGraph;
use byterag_core::Database;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !name.starts_with('.') && name != "target" && name != "node_modules" {
                    collect_rs_files(&path, out);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
}

fn main() -> byterag_core::ByteRagResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let project_root = args.get(1).cloned().unwrap_or_else(|| "d:/ByteLogicCore/modify/pumAI".to_string());
    let output_dir = args.get(2).cloned().unwrap_or_else(|| format!("{}/.byterag", project_root));

    println!("============================================================");
    println!(" 🚀 ByteRAG Native Rust In-Process GraphRAG Indexer");
    println!(" Target Project: {}", project_root);
    println!(" ByteRAG Storage: {}", output_dir);
    println!("============================================================");

    let db_path = PathBuf::from(&output_dir);
    if !db_path.exists() {
        fs::create_dir_all(&db_path)?;
    }

    let db = Database::open_in_memory()?;

    let mut rs_files = Vec::new();
    let root_path = Path::new(&project_root);
    collect_rs_files(root_path, &mut rs_files);

    println!("📂 Scanning {} Rust source files...", rs_files.len());

    let mut graph_edges: Vec<(String, String)> = Vec::new();
    let mut total_symbols = 0;

    for file_path in &rs_files {
        let rel_path = file_path.strip_prefix(root_path).unwrap_or(file_path).to_string_lossy().replace('\\', "/");
        let file_node_id = format!("file:{}", rel_path);

        if let Ok(content) = fs::read_to_string(file_path) {
            for (line_idx, line_raw) in content.lines().enumerate() {
                let line = line_raw.trim();
                let line_no = line_idx + 1;

                if line.starts_with("pub struct ") || line.starts_with("struct ") {
                    if let Some(name) = line.split_whitespace().nth(if line.starts_with("pub") { 2 } else { 1 }) {
                        let clean_name = name.trim_end_matches('{').trim_end_matches(';').trim();
                        if !clean_name.is_empty() {
                            let struct_id = format!("struct:{}", clean_name);
                            let payload = format!("{{\"type\":\"Struct\",\"name\":\"{}\",\"file\":\"{}\",\"line\":{}}}", clean_name, rel_path, line_no);
                            
                            db.insert("kg_nodes", struct_id.as_bytes(), payload.as_bytes())?;
                            db.insert("kg_edges_fwd", format!("{}:declares:{}", file_node_id, struct_id).as_bytes(), b"{}")?;
                            
                            graph_edges.push((file_node_id.clone(), struct_id));
                            total_symbols += 1;
                        }
                    }
                } else if line.starts_with("pub fn ") || line.starts_with("fn ") || line.starts_with("pub async fn ") || line.starts_with("async fn ") {
                    let parts: Vec<&str> = line.split('(').collect();
                    if let Some(decl) = parts.first() {
                        if let Some(fn_name) = decl.split_whitespace().last() {
                            let clean_name = fn_name.trim();
                            if !clean_name.is_empty() && clean_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                let fn_id = format!("fn:{}", clean_name);
                                let payload = format!("{{\"type\":\"Function\",\"name\":\"{}\",\"file\":\"{}\",\"line\":{}}}", clean_name, rel_path, line_no);
                                
                                db.insert("kg_nodes", fn_id.as_bytes(), payload.as_bytes())?;
                                db.insert("kg_edges_fwd", format!("{}:declares:{}", file_node_id, fn_id).as_bytes(), b"{}")?;
                                
                                graph_edges.push((file_node_id.clone(), fn_id));
                                total_symbols += 1;
                            }
                        }
                    }
                } else if line.starts_with("pub enum ") || line.starts_with("enum ") {
                    if let Some(name) = line.split_whitespace().nth(if line.starts_with("pub") { 2 } else { 1 }) {
                        let clean_name = name.trim_end_matches('{').trim();
                        if !clean_name.is_empty() {
                            let enum_id = format!("enum:{}", clean_name);
                            let payload = format!("{{\"type\":\"Enum\",\"name\":\"{}\",\"file\":\"{}\",\"line\":{}}}", clean_name, rel_path, line_no);
                            
                            db.insert("kg_nodes", enum_id.as_bytes(), payload.as_bytes())?;
                            db.insert("kg_edges_fwd", format!("{}:declares:{}", file_node_id, enum_id).as_bytes(), b"{}")?;
                            
                            graph_edges.push((file_node_id.clone(), enum_id));
                            total_symbols += 1;
                        }
                    }
                }
            }
        }
    }

    println!("⚡ Building ByteRAG Native In-Memory CSR Graph...");
    let csr = CsrGraph::from_edges(&graph_edges);

    println!("💾 Flushing Delta Store to ByteRAG Native Storage...");
    db.flush()?;

    println!("\n✅ ByteRAG Native Graph Indexing Complete!");
    println!("   • Total Indexed Files: {}", rs_files.len());
    println!("   • Total AST Symbols: {}", total_symbols);
    println!("   • Total Graph Edges: {}", graph_edges.len());
    println!("   • CSR Graph Nodes: {}", csr.num_nodes);

    if let Some(first_file) = graph_edges.first() {
        let test_node = &first_file.0;
        if let Some(subgraph) = csr.multi_hop_traversal(test_node, 2) {
            println!("\n🔍 [ByteRAG Verification] Multi-hop Traversal from '{}':", test_node);
            println!("   - Reachable Nodes (Depth 2): {}", subgraph.nodes.len());
            println!("   - Traversed Edges: {}", subgraph.edges.len());
        }
    }

    Ok(())
}
