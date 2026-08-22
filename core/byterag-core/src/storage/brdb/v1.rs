//! `.brdb` v1 — whole-blob pack.

use super::format::{FORMAT_VERSION_V1, MAGIC};
use crate::error::{ByteRagError, ByteRagResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Write a v1 whole-blob `.brdb` pack.
pub fn write_v1(path: &Path, snapshot_bytes: &[u8]) -> ByteRagResult<()> {
    let mut f = File::create(path)?;
    f.write_all(MAGIC)?;
    f.write_all(&FORMAT_VERSION_V1.to_le_bytes())?;
    f.write_all(&(snapshot_bytes.len() as u64).to_le_bytes())?;
    f.write_all(snapshot_bytes)?;
    f.sync_all()?;
    Ok(())
}

/// Read a v1 whole-blob `.brdb` pack.
pub fn read_v1(path: &Path) -> ByteRagResult<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(ByteRagError::InvalidOperation {
            message: "invalid .brdb magic".into(),
            context: "read_v1".into(),
        });
    }
    let mut ver = [0u8; 4];
    f.read_exact(&mut ver)?;
    let version = u32::from_le_bytes(ver);
    if version != FORMAT_VERSION_V1 {
        return Err(ByteRagError::InvalidOperation {
            message: format!("expected .brdb v1, got {version}"),
            context: "read_v1".into(),
        });
    }
    let mut len_buf = [0u8; 8];
    f.read_exact(&mut len_buf)?;
    let len = u64::from_le_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    f.read_exact(&mut payload)?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.brdb");
        let data = b"hello-snapshot".to_vec();
        write_v1(&path, &data).unwrap();
        assert_eq!(read_v1(&path).unwrap(), data);
    }
}
