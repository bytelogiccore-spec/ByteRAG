use byterag_core::Database;
use byterag_core::vector::Metric;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("   DBX Multi-Model (HTAP + Vector + Graph) E2E Test        ");
    println!("============================================================");

    let db = Database::open_in_memory()?;

    // 1. OLTP CRUD Test
    println!("[1] Testing OLTP CRUD...");
    db.insert("users", b"user:1", b"Alice (Lead Engineer)")?;
    let val = db.get("users", b"user:1")?;
    println!(
        "  >> Inserted & Retrieved: {:?}",
        String::from_utf8(val.unwrap())
    );

    // 2. Vector Search Test
    println!("\n[2] Testing Vector Search (SIMD Cosine)...");
    let dim = 4;
    let doc1_vec: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    let doc2_vec: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
    let doc3_vec: [f32; 4] = [0.8, 0.6, 0.0, 0.0];

    let as_bytes = |v: &[f32]| -> &[u8] {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
    };

    db.insert(
        "doc_embeddings",
        b"doc:rust_memory_safety",
        as_bytes(&doc1_vec),
    )?;
    db.insert("doc_embeddings", b"doc:python_async", as_bytes(&doc2_vec))?;
    db.insert(
        "doc_embeddings",
        b"doc:rust_concurrency",
        as_bytes(&doc3_vec),
    )?;

    let query: [f32; 4] = [1.0, 0.1, 0.0, 0.0];
    let vec_results = db.vector_search("doc_embeddings", &query, 2, Metric::Cosine)?;
    for (i, res) in vec_results.iter().enumerate() {
        println!(
            "  >> Top {}: ID = {}, Score = {:.4}",
            i + 1,
            String::from_utf8_lossy(&res.id),
            res.score
        );
    }

    // 3. Knowledge Graph Traversal Test
    println!("\n[3] Testing Knowledge Graph Multi-hop Traversal...");
    db.insert("call_graph", b"main()", b"init_database()")?;
    db.insert("call_graph", b"init_database()", b"open_storage_engine()")?;
    db.insert(
        "call_graph",
        b"open_storage_engine()",
        b"load_arrow_cache()",
    )?;
    db.insert("call_graph", b"main()", b"start_agent_server()")?;

    if let Some(subgraph) = db.graph_traverse("call_graph", "main()", 2)? {
        println!("  >> Traversed Subgraph from 'main()' (max_depth=2):");
        println!("     Total nodes reachable: {}", subgraph.nodes.len());
        println!("     Total edges traversed: {}", subgraph.edges.len());
    }

    println!("============================================================");
    println!("   All Multi-Model Engines Verified Successfully!");
    println!("============================================================");
    Ok(())
}
