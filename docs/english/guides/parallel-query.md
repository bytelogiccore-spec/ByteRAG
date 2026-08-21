---
layout: default
title: Parallel Query
parent: English
nav_order: 34
---

# Parallel Query Execution
{: .no_toc }

Process large datasets in parallel using Rayon thread pools.
{: .fs-6 .fw-300 }

---

## Overview

DBX's parallel query executor processes multiple RecordBatches concurrently. Parallelization only activates when data is large enough, avoiding overhead on small datasets.

```
Small (< 1,000 rows):  Sequential    → No overhead
Large (≥ 1,000 rows):  Parallel      → Multi-core utilization
```

---

## Supported Operations

| Operation | Method | Description |
|-----------|--------|-------------|
| **Filter** | `par_filter()` | Parallel row filtering by predicate |
| **Aggregate** | `par_aggregate()` | Parallel SUM, COUNT, AVG, MIN, MAX |
| **Projection** | `par_project()` | Parallel column extraction |

---

## Usage

```rust
use byterag_core::sql::executor::parallel_query::{
    ParallelQueryExecutor, AggregateType
};

let executor = ParallelQueryExecutor::new();  // default: parallel above 1000 rows

// Parallel aggregation
let result = executor.par_aggregate(&batches, 0, AggregateType::Sum)?;
println!("Sum: {}, Count: {}", result.value, result.count);

// Custom configuration
let executor = ParallelQueryExecutor::new()
    .with_min_rows(5000)         // parallel above 5000 rows
    .with_threshold(4)           // requires 4+ batches
    .with_thread_pool(pool);     // custom thread pool
```

---

## Parallelization Criteria

Parallel execution activates when **both** conditions are met:

1. **Batch count** ≥ `parallel_threshold` (default 2)
2. **Total rows** ≥ `min_rows_for_parallel` (default 1,000)

---

## Performance

| Data Size | Sequential | Parallel | Note |
|-----------|:----------:|:--------:|------|
| 150 rows | 431 ns | 32.5 µs | 🚫 Parallel slower → sequential fallback |
| 10,000 rows | ~50 µs | ~15 µs | ✅ Parallel faster |
| 1M rows | ~5 ms | ~1.2 ms | 🔥 4x improvement |

---

## Next Steps

- [Plan Cache Guide](plan-cache) — Optimize repeated SQL execution
- [WAL Recovery Guide](wal-recovery) — WAL partitioning synergy

