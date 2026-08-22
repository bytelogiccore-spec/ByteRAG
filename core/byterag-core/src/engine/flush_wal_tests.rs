//! Flush + WAL truncate capacity / data tests (V-CAP, V-DATA).

#[cfg(test)]
mod flush_wal_trim_tests {
    use crate::Database;
    use std::time::Instant;

    #[test]
    fn flush_shrinks_wal_log_and_preserves_data() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path()).unwrap();

        const N: usize = 3000;
        for i in 0..N {
            let key = format!("k{i:05}");
            let val = format!("value-{i}-{}", "x".repeat(32));
            db.insert("items", key.as_bytes(), val.as_bytes())
                .unwrap();
        }

        let wal_path = dir.path().join("wal.log");
        let size_before = std::fs::metadata(&wal_path).unwrap().len();
        assert!(
            size_before > 50_000,
            "expected a large WAL before flush, got {size_before}"
        );

        db.flush().unwrap();

        let size_after = std::fs::metadata(&wal_path).unwrap().len();
        assert!(
            size_after < size_before,
            "WAL must shrink: before={size_before} after={size_after}"
        );
        assert!(
            size_after <= size_before / 20 || size_after < 4096,
            "WAL should be near checkpoint size: before={size_before} after={size_after}"
        );

        for i in [0usize, N / 2, N - 1] {
            let key = format!("k{i:05}");
            let got = db.get("items", key.as_bytes()).unwrap().unwrap();
            assert!(
                got.starts_with(format!("value-{i}-").as_bytes()),
                "missing/corrupt key {key}"
            );
        }
        assert_eq!(db.count("items").unwrap(), N);
    }

    #[test]
    fn flush_then_reopen_preserves_data() {
        let dir = tempfile::tempdir().unwrap();
        {
            let db = Database::open(dir.path()).unwrap();
            db.insert("t", b"a", b"1").unwrap();
            db.insert("t", b"b", b"2").unwrap();
            db.flush().unwrap();
        }
        let db = Database::open(dir.path()).unwrap();
        assert_eq!(db.get("t", b"a").unwrap().unwrap(), b"1");
        assert_eq!(db.get("t", b"b").unwrap().unwrap(), b"2");
    }

    /// V-INS / V-FLUSH: record timings; insert path should stay in a sane band.
    #[test]
    fn insert_and_flush_timing_smoke() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path()).unwrap();

        const N: usize = 2000;
        // warmup
        for i in 0..100 {
            db.insert("w", format!("{i}").as_bytes(), b"x").unwrap();
        }
        db.flush().unwrap();

        let t0 = Instant::now();
        for i in 0..N {
            db.insert("perf", format!("k{i}").as_bytes(), b"payload-bytes-here")
                .unwrap();
        }
        let insert_ms = t0.elapsed().as_millis();

        let t1 = Instant::now();
        db.flush().unwrap();
        let flush_ms = t1.elapsed().as_millis();

        // Soft ceilings — CI machines vary; catch pathological regressions only.
        assert!(
            insert_ms < 30_000,
            "insert {N} took {insert_ms}ms (suspiciously slow)"
        );
        assert!(
            flush_ms < 60_000,
            "flush after {N} inserts took {flush_ms}ms (suspiciously slow)"
        );

        eprintln!("V-INS insert_ms={insert_ms} V-FLUSH flush_ms={flush_ms} n={N}");
    }
}
