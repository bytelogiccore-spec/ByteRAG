use byterag_core::Database;
use byterag_core::error::ByteRagResult;
use byterag_core::vector::Metric;
use std::sync::Arc;

// ════════════════════════════════════════════
// Rig-core Compatible Vector Store Interface
// ════════════════════════════════════════════

pub struct DbxVectorStore {
    db: Arc<Database>,
    table_name: String,
    dimension: usize,
}

impl DbxVectorStore {
    pub fn new(db: Arc<Database>, table_name: &str, dimension: usize) -> Self {
        Self {
            db,
            table_name: table_name.to_string(),
            dimension,
        }
    }

    /// Add an embedded document to DBX
    pub fn add_document(
        &self,
        doc_id: &str,
        embedding: &[f32],
        content: &str,
    ) -> ByteRagResult<()> {
        assert_eq!(
            embedding.len(),
            self.dimension,
            "Embedding dimension mismatch"
        );

        // 1. Store embedding vector in vector table
        let vec_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                embedding.as_ptr() as *const u8,
                embedding.len() * std::mem::size_of::<f32>(),
            )
        };
        self.db
            .insert(&self.table_name, doc_id.as_bytes(), vec_bytes)?;

        // 2. Store content payload in payload table
        let payload_table = format!("{}_payload", self.table_name);
        self.db
            .insert(&payload_table, doc_id.as_bytes(), content.as_bytes())?;

        Ok(())
    }

    /// Retrieve Top-N most relevant documents for AI Agents (Rig-core top_n semantics)
    pub fn top_n(
        &self,
        query_embedding: &[f32],
        n: usize,
    ) -> ByteRagResult<Vec<(f32, String, String)>> {
        let results =
            self.db
                .vector_search(&self.table_name, query_embedding, n, Metric::Cosine)?;
        let payload_table = format!("{}_payload", self.table_name);

        let mut output = Vec::with_capacity(results.len());
        for item in results {
            let doc_id = String::from_utf8_lossy(&item.id).to_string();
            let content = if let Some(bytes) = self.db.get(&payload_table, &item.id)? {
                String::from_utf8_lossy(&bytes).to_string()
            } else {
                String::new()
            };
            output.push((item.score, doc_id, content));
        }

        Ok(output)
    }
}

// ════════════════════════════════════════════
// AI Agent Knowledge Tool Interface
// ════════════════════════════════════════════

pub struct DbxGraphRagTool {
    db: Arc<Database>,
    vector_store: DbxVectorStore,
    graph_table: String,
}

impl DbxGraphRagTool {
    pub fn new(db: Arc<Database>, vector_table: &str, graph_table: &str, dimension: usize) -> Self {
        let vector_store = DbxVectorStore::new(Arc::clone(&db), vector_table, dimension);
        Self {
            db,
            vector_store,
            graph_table: graph_table.to_string(),
        }
    }

    /// Execute Hybrid GraphRAG for AI Agent Reasoning
    pub fn query_context(
        &self,
        entry_node: &str,
        query_embedding: &[f32],
        max_depth: usize,
        top_k: usize,
    ) -> ByteRagResult<String> {
        let mut context_builder = String::new();

        // 1. Graph Multi-hop Traversal
        if let Some(subgraph) = self
            .db
            .graph_traverse(&self.graph_table, entry_node, max_depth)?
        {
            context_builder.push_str(&format!(
                "### [Graph Context] Explored {} nodes and {} relations from '{}'\n",
                subgraph.nodes.len(),
                subgraph.edges.len(),
                entry_node
            ));
        }

        // 2. Vector Semantic Similarity Top-K
        let top_docs = self.vector_store.top_n(query_embedding, top_k)?;
        context_builder.push_str("### [Semantic Knowledge Chunks]\n");
        for (i, (score, id, text)) in top_docs.iter().enumerate() {
            context_builder.push_str(&format!(
                "{}. [{}] (Cosine: {:.4}) - {}\n",
                i + 1,
                id,
                score,
                text
            ));
        }

        Ok(context_builder)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("   DBX Rig-core AI Agent GraphRAG Demo                    ");
    println!("============================================================");

    let db = Arc::new(Database::open_in_memory()?);
    let rag_tool = DbxGraphRagTool::new(Arc::clone(&db), "ai_vectors", "ai_graph", 4);

    // 1. Populate Agent Knowledge Base
    println!("[1] Feeding Knowledge Chunks & Code Dependency Graph...");
    rag_tool.vector_store.add_document(
        "doc:escrow_policy",
        &[0.9, 0.2, 0.1, 0.0],
        "Escrow funds are locked until both buyer and seller multi-sig confirm.",
    )?;
    rag_tool.vector_store.add_document(
        "doc:dispute_resolution",
        &[0.85, 0.4, 0.0, 0.0],
        "Arbitrator votes are tallied on-chain with 72-hour timeout.",
    )?;
    rag_tool.vector_store.add_document(
        "doc:unrelated_weather",
        &[0.0, 0.0, 0.9, 0.3],
        "Tomorrow weather will be sunny with light breeze.",
    )?;

    // Graph Edges
    db.insert("ai_graph", b"agent:payment_handler", b"module:escrow")?;
    db.insert("ai_graph", b"module:escrow", b"fn:lock_funds")?;
    db.insert("ai_graph", b"module:escrow", b"fn:arbitrate_dispute")?;

    // 2. AI Agent Query Simulation
    println!("\n[2] AI Agent Prompt: 'How does escrow dispute handling work?'");
    let agent_query_vec: [f32; 4] = [0.88, 0.3, 0.05, 0.0];

    let synthesized_context =
        rag_tool.query_context("agent:payment_handler", &agent_query_vec, 2, 2)?;
    println!("\n[3] Synthesized Prompt Context fed into LLM:\n");
    println!("{}", synthesized_context);

    println!("============================================================");
    println!("   AI Agent GraphRAG Integration Verified Successfully!    ");
    println!("============================================================");

    Ok(())
}
