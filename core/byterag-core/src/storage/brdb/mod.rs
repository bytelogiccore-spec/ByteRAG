//! `.brdb` pack format — portable snapshot pack for ByteRAG databases.

pub mod format;
pub mod v1;
pub mod v2;

pub use format::{FORMAT_VERSION_V1, FORMAT_VERSION_V2, MAGIC};
pub use v1::{read_v1, write_v1};
pub use v2::{read_frame_v2, write_v2, BrdbV2Reader};

use crate::error::{ByteRagError, ByteRagResult};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Detect pack version from file header.
pub fn peek_version(path: &Path) -> ByteRagResult<u32> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(ByteRagError::InvalidOperation {
            message: format!("invalid .brdb magic: {:?}", magic),
            context: "expected BRDB".into(),
        });
    }
    let mut ver = [0u8; 4];
    f.read_exact(&mut ver)?;
    Ok(u32::from_le_bytes(ver))
}

/// Read pack payload as opaque snapshot bytes (v1 whole blob, or v2 all frames concatenated in TOC order).
pub fn read_snapshot_bytes(path: &Path) -> ByteRagResult<Vec<u8>> {
    match peek_version(path)? {
        FORMAT_VERSION_V1 => read_v1(path),
        FORMAT_VERSION_V2 => {
            let reader = BrdbV2Reader::open(path)?;
            reader.read_all_concat()
        }
        v => Err(ByteRagError::InvalidOperation {
            message: format!("unsupported .brdb version: {v}"),
            context: "read_snapshot_bytes".into(),
        }),
    }
}
