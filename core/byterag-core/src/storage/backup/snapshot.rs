//! Database snapshot save/load implementation

use crate::engine::Database;
use crate::engine::metadata::SchemaMetadata;
pub use crate::engine::snapshot::DatabaseSnapshot;
use crate::engine::snapshot::TableData;
use crate::error::{ByteRagError, ByteRagResult};
use arrow::datatypes::Schema;
use std::path::Path;
use std::sync::Arc;

impl Database {
    /// Save in-memory database to file
    ///
    /// Only works for in-memory databases. Returns error for file-based DBs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use byterag_core::Database;
    ///
    /// # fn main() -> byterag_core::ByteRagResult<()> {
    /// let db = Database::open_in_memory()?;
    /// db.execute_sql("CREATE TABLE users (id INT, name TEXT)")?;
    /// db.execute_sql("INSERT INTO users VALUES (1, 'Alice')")?;
    ///
    /// // Save to file
    /// db.save_to_file("backup.json")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> ByteRagResult<()> {
        // 1. Check if this is an in-memory DB
        if !self.is_in_memory() {
            return Err(ByteRagError::InvalidOperation {
                message: "save_to_file only works for in-memory databases".to_string(),
                context: "Use flush() for file-based databases".to_string(),
            });
        }

        // 2. Create snapshot
        let snapshot = self.create_snapshot()?;

        // 3. Serialize to JSON
        let json = serde_json::to_string_pretty(&snapshot)
            .map_err(|e| ByteRagError::Serialization(e.to_string()))?;

        // 4. Write to file
        std::fs::write(path, json)?;

        Ok(())
    }

    /// Export a durable `.brdb` pack (v1 whole-blob). Calls [`Self::flush`] first.
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> ByteRagResult<()> {
        self.export_to_file_version(path, crate::storage::brdb::FORMAT_VERSION_V1)
    }

    /// Export `.brdb` with explicit format version (1 = whole blob, 2 = seekable frames).
    pub fn export_to_file_version<P: AsRef<Path>>(
        &self,
        path: P,
        format_version: u32,
    ) -> ByteRagResult<()> {
        self.flush()?;
        let snapshot = self.create_snapshot()?;
        let bytes = bincode::serialize(&snapshot)
            .map_err(|e| ByteRagError::Serialization(e.to_string()))?;
        match format_version {
            crate::storage::brdb::FORMAT_VERSION_V1 => {
                crate::storage::brdb::write_v1(path.as_ref(), &bytes)
            }
            crate::storage::brdb::FORMAT_VERSION_V2 => {
                crate::storage::brdb::write_v2(path.as_ref(), &bytes)
            }
            v => Err(ByteRagError::InvalidOperation {
                message: format!("unsupported .brdb version {v}"),
                context: "export_to_file_version".into(),
            }),
        }
    }

    /// Open a database from a `.brdb` pack into a new in-memory instance.
    pub fn open_from_file<P: AsRef<Path>>(path: P) -> ByteRagResult<Self> {
        let bytes = crate::storage::brdb::read_snapshot_bytes(path.as_ref())?;
        let snapshot: DatabaseSnapshot = bincode::deserialize(&bytes)
            .map_err(|e| ByteRagError::Serialization(e.to_string()))?;
        let db = Self::open_in_memory()?;
        db.restore_snapshot(snapshot)?;
        Ok(db)
    }

    /// Load database from file into in-memory database
    ///
    /// Creates a new in-memory DB and loads all data from file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use byterag_core::Database;
    ///
    /// # fn main() -> byterag_core::ByteRagResult<()> {
    /// // Load from file
    /// let db = Database::load_from_file("backup.json")?;
    ///
    /// // Query data
    /// let results = db.execute_sql("SELECT * FROM users")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> ByteRagResult<Self> {
        // 1. Read file
        let json = std::fs::read_to_string(path)?;

        // 2. Deserialize snapshot
        let snapshot: DatabaseSnapshot =
            serde_json::from_str(&json).map_err(|e| ByteRagError::Serialization(e.to_string()))?;

        // 3. Create new in-memory DB
        let db = Self::open_in_memory()?;

        // 4. Restore snapshot
        db.restore_snapshot(snapshot)?;

        Ok(db)
    }

    /// Check if this is an in-memory database (no file persistence)
    fn is_in_memory(&self) -> bool {
        self.file_wos.is_none()
    }

    /// Create a snapshot of the current database state
    fn create_snapshot(&self) -> ByteRagResult<DatabaseSnapshot> {
        let mut snapshot = DatabaseSnapshot::new();

        // 1. Capture schemas
        let schemas = self.table_schemas.read().unwrap();
        for (table_name, schema) in schemas.iter() {
            let metadata = SchemaMetadata::from(schema.as_ref());
            snapshot.schemas.insert(table_name.clone(), metadata);
        }
        drop(schemas);

        // 2. Capture indexes
        let indexes = self.index_registry.read().unwrap();
        snapshot.indexes = indexes.clone();
        drop(indexes);

        // 3. Capture table data — prefer table_names() (delta + WOS)
        let mut table_list = self.table_names()?;
        for entry in self.row_counters.iter() {
            let t = entry.key().clone();
            if !table_list.contains(&t) {
                table_list.push(t);
            }
        }

        for table_name in table_list {
            if table_name.starts_with("__meta__") {
                continue;
            }

            let entries = self.scan(&table_name)?;
            snapshot.tables.insert(table_name, TableData { entries });
        }

        // 4. Capture row counters
        for entry in self.row_counters.iter() {
            let table = entry.key().clone();
            let counter = entry.value().load(std::sync::atomic::Ordering::SeqCst);
            snapshot.row_counters.insert(table, counter);
        }

        Ok(snapshot)
    }

    /// Restore database state from snapshot
    fn restore_snapshot(&self, snapshot: DatabaseSnapshot) -> ByteRagResult<()> {
        // 1. Validate version
        if snapshot.version != DatabaseSnapshot::CURRENT_VERSION {
            return Err(ByteRagError::InvalidOperation {
                message: format!("Unsupported snapshot version: {}", snapshot.version),
                context: format!("Expected version {}", DatabaseSnapshot::CURRENT_VERSION),
            });
        }

        // 2. Restore schemas (both table_schemas and schemas for compatibility)
        let mut table_schemas = self.table_schemas.write().unwrap();
        let mut schemas = self.schemas.write().unwrap();
        for (table_name, metadata) in snapshot.schemas {
            let schema =
                Arc::new(Schema::try_from(metadata).map_err(|e| {
                    ByteRagError::Schema(format!("Failed to restore schema: {}", e))
                })?);
            table_schemas.insert(table_name.clone(), schema.clone());
            schemas.insert(table_name, schema);
        }
        drop(table_schemas);
        drop(schemas);

        // 3. Restore indexes
        let mut indexes = self.index_registry.write().unwrap();
        *indexes = snapshot.indexes;
        drop(indexes);

        // 4. Restore table data
        for (table_name, table_data) in snapshot.tables {
            for (key, value) in table_data.entries {
                self.wos_for_table(&table_name)
                    .insert(&table_name, &key, &value)?;
            }
        }

        // 5. Restore row counters
        for (table, count) in snapshot.row_counters {
            self.row_counters
                .insert(table, std::sync::atomic::AtomicUsize::new(count));
        }

        Ok(())
    }
}

// ════════════════════════════════════════════
// DatabaseSnapshot Trait Implementation
// ════════════════════════════════════════════

impl crate::traits::DatabaseSnapshot for Database {
    fn save_to_file(&self, path: &str) -> ByteRagResult<()> {
        // Reuse existing implementation
        Database::save_to_file(self, path)
    }

    fn load_from_file(path: &str) -> ByteRagResult<Self> {
        // Reuse existing implementation
        Database::load_from_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_memory() {
        let db = Database::open_in_memory().unwrap();
        assert!(db.is_in_memory());
    }

    #[test]
    fn test_brdb_export_open_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let pack = dir.path().join("store.brdb");
        {
            let db = Database::open(&dir.path().join("live")).unwrap();
            db.insert("users", b"u1", b"Alice").unwrap();
            db.insert("users", b"u2", b"Bob").unwrap();
            db.export_to_file(&pack).unwrap();
            let wal = std::fs::metadata(dir.path().join("live").join("wal.log"))
                .unwrap()
                .len();
            assert!(wal < 4096, "export should flush+trim WAL, size={wal}");
        }
        let db = Database::open_from_file(&pack).unwrap();
        assert_eq!(db.get("users", b"u1").unwrap().unwrap(), b"Alice");
        assert_eq!(db.get("users", b"u2").unwrap().unwrap(), b"Bob");
    }

    #[test]
    fn test_brdb_v2_export_open_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let pack = dir.path().join("store-v2.brdb");
        {
            let db = Database::open(&dir.path().join("live")).unwrap();
            for i in 0..100 {
                db.insert("t", format!("k{i}").as_bytes(), b"v").unwrap();
            }
            db.export_to_file_version(&pack, crate::storage::brdb::FORMAT_VERSION_V2)
                .unwrap();
        }
        let db = Database::open_from_file(&pack).unwrap();
        assert_eq!(db.count("t").unwrap(), 100);
    }
}
