# ByteRAG — Ultra-Fast Embedded Multi-Model Database for AI & GraphRAG

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/bytelogiccore-spec/ByteRAG)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://bytelogiccore-spec.github.io/ByteRAG/)

---

**ByteRAG** is a high-performance, pure-Rust embedded database engine designed from the ground up for **AI Agents, Vector Search, Knowledge Graphs (GraphRAG), and HTAP workloads**. Built with an ultra-low latency **5-Tier Hybrid Storage** architecture (Delta → Cache → WOS → Index → ROS), ByteRAG unifies conversational context caching, vector similarity search, knowledge graph traversal, and analytical queries in a single embeddable binary.

---

## ⚡ The ByteRAG Edge: Why Choose ByteRAG?

### 🧠 1. All-in-One Engine for AI & GraphRAG
Eliminate the need to operate 4 different databases (Vector DB + SQL + Graph DB + In-Memory Cache):
- **SIMD-Accelerated Vector Search**: High-throughput cosine similarity, L2, and dot-product calculations with AVX-512 / AVX2 support.
- **CSR Knowledge Graph Traversal**: Low-latency neighbor exploration for multi-hop GraphRAG reasoning.
- **Unified Context Synthesis**: Query relational metadata, vector embeddings, and graph relations in a single pipeline.

### 🚄 2. 5-Tier Hybrid Storage Architecture
Flow data dynamically across 5 optimized tiers:
- **Tier 1 (Delta)**: Lock-free in-memory write buffer for sub-millisecond conversation and event streams.
- **Tier 2 (Cache)**: Apache Arrow-based columnar cache for instantaneous memory queries.
- **Tier 3 (WOS)**: Write-Optimized Store with MVCC snapshot isolation.
- **Tier 4 (Index)**: High-speed Bloom filters & Hash indexes for near-zero latency probes.
- **Tier 5 (ROS)**: Compact columnar Parquet storage for petabyte-scale knowledge archives.

### 🏎️ 3. Pure Rust & Embeddable
- **Zero Heavy Runtime / Zero Docker Required**: Embed directly into your application binary or Python/Node.js/C# environment.
- **Safe Concurrency (MVCC + Row Latches)**: Lock-free lock managers and distributed coordination (DLM) powered by QUIC transport.

---

## 🏗️ Architecture Overview

```mermaid
graph TD
    A[AI Agent / LLM App] -->|Store / Retrieve| B[ByteRAG Engine]
    
    subgraph "Unified Storage Engine"
        B -->|Write Stream| T1[Tier 1: Delta Memory]
        T1 -->|Flush| T2[Tier 2: Arrow Cache]
        T2 -->|Persist| T3[Tier 3: WOS - SSD]
        T3 -->|Compact| T4[Tier 4: Index & Bloom]
        T4 -->|Archive| T5[Tier 5: ROS - Parquet]
    end
    
    subgraph "AI & Analytics Modules"
        B --> V[Vector Search (SIMD/HNSW)]
        B --> G[Knowledge Graph (CSR GraphRAG)]
        B --> S[SQL Optimizer (Arrow Columnar)]
    end
```

---

## 🚀 Quick Start (Rust)

```rust
use byterag_core::Database;

fn main() -> byterag_core::ByteRagResult<()> {
    // Open an embedded database in memory or on disk
    let db = Database::open_in_memory()?;

    // Fast KV CRUD
    db.insert("documents", b"doc:1", b"{\"content\": \"ByteRAG Engine\", \"tags\": [\"ai\", \"rag\"]}")?;
    let val = db.get("documents", b"doc:1")?;

    // Execute Vector & SQL queries
    let results = db.execute_sql("SELECT * FROM documents")?;

    Ok(())
}
```

---

## 🤝 Support & Contributing

ByteRAG is an open-source project by **ByteLogicCore**.
- ⭐️ **Star us** on GitHub: [ByteRAG](https://github.com/bytelogiccore-spec/ByteRAG)
- 🐛 **Report issues** or request features via GitHub Issues.

---

**Made with ❤️ in Rust for Next-Gen AI & GraphRAG Applications.**

