# byterag-ffi

[![Crates.io](https://img.shields.io/crates/v/byterag-ffi.svg)](https://crates.io/crates/byterag-ffi)

C FFI bindings for the DBX embedded database engine.

This crate provides a C-compatible interface (`cdylib` + `staticlib`) to the `byterag-core` engine, enabling integration with C, C++, C#, Python, and Node.js.

## Exported Functions

| Function | Description |
|----------|-------------|
| `byterag_open(path)` | Open a file-based database |
| `byterag_open_in_memory()` | Open an in-memory database |
| `byterag_insert(db, table, key, value)` | Insert a key-value pair |
| `byterag_get(db, table, key)` | Get value by key |
| `byterag_delete(db, table, key)` | Delete a key |
| `byterag_close(db)` | Close and free resources |
| `byterag_begin_transaction(db)` | Start a transaction |
| `byterag_transaction_commit(tx)` | Commit a transaction |

## Building

```bash
cargo build --release -p byterag-ffi
```

Produces `byterag_ffi.dll` (Windows) / `libbyterag_ffi.so` (Linux) / `libbyterag_ffi.dylib` (macOS).

## License

MIT License

