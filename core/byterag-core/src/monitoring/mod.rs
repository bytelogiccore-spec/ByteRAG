//! ByteRAG Monitoring & Observability Module
//!
//! Provides Prometheus-compatible metrics collection and export for ByteRAG.
//!
//! ## Metrics Categories
//! - **Operation Counters**: INSERT, GET, DELETE, SQL query counts
//! - **Tier Hit Rates**: Delta Store, Columnar Cache, WOS hit/miss rates
//! - **Sharding Stats**: Scatter reads/writes
//! - **Partition Stats**: Partition pruning hits
//! - **WAL Stats**: Append and compaction counts
//! - **Latency Histograms**: Query and insert latency distribution
//!
//! ## Usage via `Database`
//!
//! ```rust
//! use byterag_core::Database;
//!
//! # fn main() -> byterag_core::ByteRagResult<()> {
//! let db = Database::open_in_memory()?;
//!
//! // Execute some operations...
//! db.insert("users", b"k1", b"v1")?;
//! db.get("users", b"k1")?;
//!
//! // Export as Prometheus text
//! let metrics_text = db.export_metrics();
//! assert!(metrics_text.contains("byterag_inserts_total"));
//!
//! // Get structured snapshot
//! let snap = db.metrics_snapshot();
//! assert_eq!(snap.inserts_total, 1);
//!
//! // Reset all metrics
//! db.reset_metrics();
//! # Ok(())
//! # }
//! ```

pub mod exporter;
pub mod histogram;
pub mod metrics;

pub use exporter::export_prometheus;
pub use metrics::{ByteRagMetrics, MetricsSnapshot};



