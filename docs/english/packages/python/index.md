---
layout: default
title: Python (byterag-py)
nav_order: 3
parent: Packages
grand_parent: English
has_children: true
---

# Python — byterag-py

[![PyPI](https://img.shields.io/pypi/v/byterag-py.svg)](https://pypi.org/project/byterag-py/)

Official Python bindings for DBX high-performance embedded database.

## Key Features

- 🚀 **Native Performance**: Rust-based PyO3 bindings
- 💾 **5-Tier Storage**: WOS → L0 → L1 → L2 → Cold Storage
- 🔒 **MVCC Transactions**: Snapshot isolation support
- 📊 **SQL Support**: Full DDL + DML support
- 🔐 **Encryption**: AES-GCM-SIV, ChaCha20-Poly1305
- 🐍 **Pythonic API**: Context Manager, Type Hints

## Quick Start

```bash
pip install byterag-py
```

```python
from byterag_py import Database

db = Database.open_in_memory()

# KV operations
db.insert("users", b"user:1", b"Alice")
value = db.get("users", b"user:1")
print(value.decode())  # Alice

# SQL operations
db.execute_sql("CREATE TABLE users (id INTEGER, name TEXT)")
db.execute_sql("INSERT INTO users VALUES (1, 'Alice')")
result = db.execute_sql("SELECT * FROM users")
print(result)

db.close()
```

## Documentation

- [Installation](installation) - Setup and configuration
- [Quick Start](quickstart) - Get started in 5 minutes
- [KV Operations](kv-operations) - Key-Value operations guide
- [SQL Guide](sql-guide) - SQL usage
- [Advanced](advanced) - Transactions, encryption, performance tuning
- [API Reference](api-reference) - Complete API documentation
- [Examples](examples) - Real-world examples

## Version Info

- **Current Version**: {{ site.byterag_version }}
- **Python Requirements**: 3.8+
- **Platform**: Windows x64 (Linux/macOS planned)

## License

MIT License

