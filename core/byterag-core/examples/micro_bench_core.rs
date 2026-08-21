use std::time::Instant;
use byterag_core::engine::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("   DBX Core Micro-Benchmark (10,000 Records)               ");
    println!("============================================================");

    // 1. Setup in-memory DB
    let db = Database::open_in_memory()?;

    let count = 10_000;
    println!("[1] Preparing {} records...", count);

    // 2. Benchmark Continuous Single INSERT
    let start_insert = Instant::now();
    for i in 0..count {
        let key = format!("key_{:07}", i);
        let val = format!("value_payload_data_for_key_{:07}", i);
        db.insert("benchmark_table", key.as_bytes(), val.as_bytes())?;
    }
    let insert_dur = start_insert.elapsed();
    let insert_ops = (count as f64) / insert_dur.as_secs_f64();
    println!("  >> INSERT 10K completed in: {:.2?} ({:.0} ops/sec)", insert_dur, insert_ops);

    // 3. Benchmark Point GET (Single Lookup)
    let start_get = Instant::now();
    let mut hit_count = 0;
    for i in 0..count {
        let key = format!("key_{:07}", i);
        if let Some(_v) = db.get("benchmark_table", key.as_bytes())? {
            hit_count += 1;
        }
    }
    let get_dur = start_get.elapsed();
    let get_ops = (count as f64) / get_dur.as_secs_f64();
    println!("  >> GET 10K completed in: {:.2?} ({:.0} ops/sec, hits: {})", get_dur, get_ops, hit_count);

    // 4. Benchmark SCAN
    let start_scan = Instant::now();
    let scan_results = db.scan("benchmark_table")?;
    let scan_dur = start_scan.elapsed();
    println!("  >> SCAN 10K completed in: {:.2?} (scanned {} items)", scan_dur, scan_results.len());

    println!("============================================================");
    println!("   Summary:");
    println!("   - INSERT: {:.3} ms ({:.1} ns/op)", insert_dur.as_secs_f64() * 1000.0, insert_dur.as_nanos() as f64 / count as f64);
    println!("   - GET:    {:.3} ms ({:.1} ns/op)", get_dur.as_secs_f64() * 1000.0, get_dur.as_nanos() as f64 / count as f64);
    println!("   - SCAN:   {:.3} ms", scan_dur.as_secs_f64() * 1000.0);
    println!("============================================================");

    Ok(())
}

