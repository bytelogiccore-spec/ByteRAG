---
layout: default
title: Node.js (byterag-py)
nav_order: 4
parent: Packages
grand_parent: English
has_children: true
---

# Node.js — byterag-py

[![npm](https://img.shields.io/npm/v/byterag-py.svg)](https://www.npmjs.com/package/byterag-py)

Official Node.js bindings for ByteRAG high-performance embedded database.

## Key Features

- 🚀 **Native Performance**: Rust-based N-API bindings
- 💾 **5-Tier Storage**: WOS → L0 → L1 → L2 → Cold Storage
- 🔒 **MVCC Transactions**: Snapshot isolation support
- 📊 **SQL Support**: Full DDL + DML support
- 🔐 **Encryption**: AES-GCM-SIV, ChaCha20-Poly1305
- 📘 **TypeScript**: Full type definitions

## Quick Start

```bash
npm install byterag-py
```

```typescript
import { Database } from 'byterag-py';

const db = Database.openInMemory();

// KV operations
db.insert('users', Buffer.from('user:1'), Buffer.from('Alice'));
const value = db.get('users', Buffer.from('user:1'));
console.log(value?.toString());  // Alice

// SQL operations
db.executeSql('CREATE TABLE users (id INTEGER, name TEXT)');
db.executeSql("INSERT INTO users VALUES (1, 'Alice')");
const result = db.executeSql('SELECT * FROM users');
console.log(result);

db.close();
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
- **Node.js Requirements**: 16+
- **Platform**: Windows x64 (Linux/macOS planned)

## License

MIT License


