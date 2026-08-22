# ByteRAG 0.3.0 — publish checklist

Implementation is complete on the working tree. **Do not publish until changes are committed and CI is green.**

## Pre-publish
1. Commit all 0.3.0 changes on `main` (or release branch).
2. Ensure `cargo test -p byterag-core --lib` gates pass (WAL trim, brdb, timing smoke).
3. Tag: `git tag v0.3.0 && git push origin v0.3.0` (if workflows are tag-triggered).

## Workflows (manual dispatch if supported)
```bash
gh workflow run "Publish → crates.io"
gh workflow run "Publish → npm"
gh workflow run "Publish → PyPI"
gh workflow run "Publish → NuGet"
```

## Verify
- crates.io: `byterag-core` / `byterag-ffi` 0.3.0
- npm: package version 0.3.0
- PyPI: `dbx_py` 0.3.0
- NuGet: `DBX.Dotnet` 0.3.0

## API reminder
- `flush()` — Delta→WOS + sync WAL truncate
- `export_to_file(path)` — flush + `.brdb` v1
- `export_to_file_version(path, 2)` — seekable frames
- `open_from_file(path)` — load pack into memory DB
