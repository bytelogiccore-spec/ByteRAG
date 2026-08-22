//! `.brdb` pack format constants and layout documentation.
//!
//! # Planned file layout
//!
//! ## v1 — whole-blob pack
//!
//! ```text
//! [MAGIC: 4 bytes "BRDB"]
//! [FORMAT_VERSION: u32 LE = 1]
//! [payload_len: u64 LE]
//! [payload: payload_len bytes]   // opaque snapshot blob
//! ```
//!
//! The entire database snapshot is stored as a single contiguous payload.
//! Suitable for small DBs and as a bootstrap until streaming export exists.
//!
//! ## v2 — TOC + zstd frames
//!
//! ```text
//! [MAGIC: 4 bytes "BRDB"]
//! [FORMAT_VERSION: u32 LE = 2]
//! [toc_offset: u64 LE]           // absolute offset of TOC
//! [frame_0: zstd-compressed chunk]
//! [frame_1: zstd-compressed chunk]
//! ...
//! [TOC]
//!   [entry_count: u32 LE]
//!   repeated entry_count times:
//!     [name_len: u16 LE][name: UTF-8]
//!     [frame_offset: u64 LE]
//!     [frame_compressed_len: u64 LE]
//!     [frame_uncompressed_len: u64 LE]
//!     [checksum: u32 LE]         // xxhash32 of uncompressed frame
//! ```
//!
//! Frames are independently decompressible; the TOC at the end enables
//! random access without scanning the whole file.

/// File magic: ASCII `BRDB`.
pub const MAGIC: &[u8; 4] = b"BRDB";

/// Format version for whole-blob packs.
pub const FORMAT_VERSION_V1: u32 = 1;

/// Format version for TOC + zstd frame packs.
pub const FORMAT_VERSION_V2: u32 = 2;
