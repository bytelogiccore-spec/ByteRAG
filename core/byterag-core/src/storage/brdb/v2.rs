//! `.brdb` v2 — TOC + independently decompressible zstd frames.

use super::format::{FORMAT_VERSION_V2, MAGIC};
use crate::error::{ByteRagError, ByteRagResult};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const DEFAULT_FRAME_TARGET: usize = 64 * 1024;

#[derive(Debug, Clone)]
pub struct TocEntry {
    pub name: String,
    pub frame_offset: u64,
    pub frame_compressed_len: u64,
    pub frame_uncompressed_len: u64,
}

/// Write snapshot bytes as v2 framed pack (chunks by DEFAULT_FRAME_TARGET).
pub fn write_v2(path: &Path, snapshot_bytes: &[u8]) -> ByteRagResult<()> {
    let mut f = File::create(path)?;
    f.write_all(MAGIC)?;
    f.write_all(&FORMAT_VERSION_V2.to_le_bytes())?;
    // placeholder toc_offset
    let toc_offset_pos = 8u64; // after magic+version
    f.write_all(&0u64.to_le_bytes())?;

    let mut entries = Vec::new();
    let mut offset = 0usize;
    let mut frame_idx = 0u32;
    while offset < snapshot_bytes.len() {
        let end = (offset + DEFAULT_FRAME_TARGET).min(snapshot_bytes.len());
        let chunk = &snapshot_bytes[offset..end];
        let compressed = zstd::encode_all(chunk, 3)
            .map_err(|e| ByteRagError::Serialization(format!("zstd compress: {e}")))?;
        let frame_offset = f.stream_position()?;
        f.write_all(&compressed)?;
        entries.push(TocEntry {
            name: format!("frame_{frame_idx}"),
            frame_offset,
            frame_compressed_len: compressed.len() as u64,
            frame_uncompressed_len: chunk.len() as u64,
        });
        offset = end;
        frame_idx += 1;
    }

    let toc_offset = f.stream_position()?;
    f.write_all(&(entries.len() as u32).to_le_bytes())?;
    for e in &entries {
        let name_bytes = e.name.as_bytes();
        f.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        f.write_all(name_bytes)?;
        f.write_all(&e.frame_offset.to_le_bytes())?;
        f.write_all(&e.frame_compressed_len.to_le_bytes())?;
        f.write_all(&e.frame_uncompressed_len.to_le_bytes())?;
        f.write_all(&0u32.to_le_bytes())?; // checksum reserved
    }

    f.seek(SeekFrom::Start(toc_offset_pos))?;
    f.write_all(&toc_offset.to_le_bytes())?;
    f.sync_all()?;
    Ok(())
}

/// Reader for v2 packs — decompresses only requested frames.
pub struct BrdbV2Reader {
    path: PathBuf,
    entries: Vec<TocEntry>,
    /// Test/observability: number of zstd decompress calls.
    pub decompress_calls: std::cell::Cell<u32>,
}

impl BrdbV2Reader {
    pub fn open(path: &Path) -> ByteRagResult<Self> {
        let mut f = File::open(path)?;
        let mut magic = [0u8; 4];
        f.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(ByteRagError::InvalidOperation {
                message: "invalid .brdb magic".into(),
                context: "BrdbV2Reader::open".into(),
            });
        }
        let mut ver = [0u8; 4];
        f.read_exact(&mut ver)?;
        if u32::from_le_bytes(ver) != FORMAT_VERSION_V2 {
            return Err(ByteRagError::InvalidOperation {
                message: "not a v2 .brdb".into(),
                context: "BrdbV2Reader::open".into(),
            });
        }
        let mut toc_off = [0u8; 8];
        f.read_exact(&mut toc_off)?;
        let toc_offset = u64::from_le_bytes(toc_off);
        f.seek(SeekFrom::Start(toc_offset))?;
        let mut nbuf = [0u8; 4];
        f.read_exact(&mut nbuf)?;
        let n = u32::from_le_bytes(nbuf) as usize;
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            let mut nl = [0u8; 2];
            f.read_exact(&mut nl)?;
            let name_len = u16::from_le_bytes(nl) as usize;
            let mut name = vec![0u8; name_len];
            f.read_exact(&mut name)?;
            let mut o = [0u8; 8];
            f.read_exact(&mut o)?;
            let mut cl = [0u8; 8];
            f.read_exact(&mut cl)?;
            let mut ul = [0u8; 8];
            f.read_exact(&mut ul)?;
            let mut cs = [0u8; 4];
            f.read_exact(&mut cs)?;
            entries.push(TocEntry {
                name: String::from_utf8_lossy(&name).into_owned(),
                frame_offset: u64::from_le_bytes(o),
                frame_compressed_len: u64::from_le_bytes(cl),
                frame_uncompressed_len: u64::from_le_bytes(ul),
            });
        }
        Ok(Self {
            path: path.to_path_buf(),
            entries,
            decompress_calls: std::cell::Cell::new(0),
        })
    }

    pub fn frame_count(&self) -> usize {
        self.entries.len()
    }

    /// Decompress a single frame by index (V-SEEK: one frame only).
    pub fn read_frame(&self, index: usize) -> ByteRagResult<Vec<u8>> {
        let e = self.entries.get(index).ok_or_else(|| ByteRagError::InvalidOperation {
            message: format!("frame index {index} out of range"),
            context: "read_frame".into(),
        })?;
        read_frame_v2(&self.path, e, &self.decompress_calls)
    }

    pub fn read_all_concat(&self) -> ByteRagResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..self.entries.len() {
            out.extend_from_slice(&self.read_frame(i)?);
        }
        Ok(out)
    }
}

pub fn read_frame_v2(
    path: &Path,
    entry: &TocEntry,
    decompress_calls: &std::cell::Cell<u32>,
) -> ByteRagResult<Vec<u8>> {
    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(entry.frame_offset))?;
    let mut comp = vec![0u8; entry.frame_compressed_len as usize];
    f.read_exact(&mut comp)?;
    decompress_calls.set(decompress_calls.get() + 1);
    zstd::decode_all(comp.as_slice())
        .map_err(|e| ByteRagError::Serialization(format!("zstd decompress: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_partial_frame_decompress() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.brdb");
        // Large enough for multiple 64KiB frames
        let mut data = Vec::new();
        for i in 0..200_000u32 {
            data.extend_from_slice(&i.to_le_bytes());
        }
        write_v2(&path, &data).unwrap();
        let reader = BrdbV2Reader::open(&path).unwrap();
        assert!(reader.frame_count() >= 2);

        let _ = reader.read_frame(0).unwrap();
        assert_eq!(reader.decompress_calls.get(), 1);

        let _ = reader.read_frame(1).unwrap();
        assert_eq!(reader.decompress_calls.get(), 2);

        // Must not equal total frames if we only read two (when more exist)
        if reader.frame_count() > 2 {
            assert!(reader.decompress_calls.get() < reader.frame_count() as u32);
        }

        let all = reader.read_all_concat().unwrap();
        assert_eq!(all, data);
    }
}
