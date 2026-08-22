---
layout: default
title: Language Bindings
nav_order: 28
parent: English
---

# Language Bindings

ByteRAG provides official bindings for multiple programming languages, allowing you to use the high-performance embedded database in your preferred development environment.

---

## 🐍 Python

High-level Python bindings with context manager support.

```python
from byterag_py import Database

with Database("my_database.db") as db:
    db.insert("users", b"user:1", b"Alice")
    value = db.get("users", b"user:1")
    print(value.decode('utf-8'))  # Output: Alice
```

**Features:**
- ✅ Context manager (`with` statement)
- ✅ Pythonic API
- ✅ Type hints support
- ✅ In-memory mode

**Installation:**
```bash
cd lang/python
pip install -e .
```

[View Python Documentation →](https://github.com/bytelogiccore-spec/ByteRAG/tree/main/lang/python)

---

## 🔷 C#/.NET

Modern .NET bindings with RAII and high-performance batch operations.

```csharp
using ByteRAG.Client;
using System.Text;

using (var db = new DbxDatabase("./my_database"))
{
    var key = Encoding.UTF8.GetBytes("user:1");
    var value = Encoding.UTF8.GetBytes("Alice");
    
    db.Insert("users", key, value);
    
    byte[] result = db.Get("users", key);
    Console.WriteLine(Encoding.UTF8.GetString(result));
}
```

**Features:**
- ✅ IDisposable pattern
- ✅ Transaction support
- ✅ Durability level control
- ✅ Thread-safe operations
- ✅ **20x faster GET** than SQLite

**Installation:**
```bash
cd lang/dotnet
dotnet build
```

[View C# Documentation →](https://github.com/bytelogiccore-spec/ByteRAG/tree/main/lang/dotnet)

---

## 🔧 C/C++

Low-level C API and modern C++17 wrapper.

### C Interface

```c
#include "ByteRAG.h"

DbxHandle* db = byterag_open("my_database.db");

byterag_insert(db, "users", 
           (uint8_t*)"user:1", 6,
           (uint8_t*)"Alice", 5);

uint8_t* value = NULL;
size_t value_len = 0;
byterag_get(db, "users", (uint8_t*)"user:1", 6, &value, &value_len);

byterag_free_value(value, value_len);
byterag_close(db);
```

### C++ Interface

```cpp
#include "ByteRAG.hpp"

using namespace ByteRAG;

auto db = Database::openInMemory();
db.insert("users", "user:1", "Alice");

if (auto value = db.getString("users", "user:1")) {
    std::cout << *value << std::endl;
}
```

**Features:**
- ✅ C API: Simple, manual memory management
- ✅ C++ API: RAII, `std::optional`, move semantics
- ✅ Modern C++17
- ✅ Exception-based error handling (C++)

**Building:**
```bash
cd lang/c
make
```

[View C/C++ Documentation →](https://github.com/bytelogiccore-spec/ByteRAG/tree/main/lang/c)

---

## 🟢 Node.js

Native Node.js bindings using N-API for maximum performance.

```javascript
const { Database } = require('byterag-node');

const db = new Database('my_database.db');

db.insert('users', Buffer.from('user:1'), Buffer.from('Alice'));

const value = db.get('users', Buffer.from('user:1'));
console.log(value.toString());  // Output: Alice

db.close();
```

**Features:**
- ✅ N-API native bindings
- ✅ Async/Promise support
- ✅ Buffer-based API
- ✅ TypeScript definitions

**Installation:**
```bash
cd lang/nodejs
npm install
```

[View Node.js Documentation →](https://github.com/bytelogiccore-spec/ByteRAG/tree/main/lang/nodejs)

---

## 📊 Performance Comparison

All language bindings provide near-native performance through zero-copy FFI.

| Language | INSERT (10k) | GET (10k) | Overhead |
|----------|--------------|-----------|----------|
| **Rust (Core)** | 25.37 ms | 17.28 ms | 0% (baseline) |
| **C/C++** | ~26 ms | ~18 ms | ~3% |
| **Python** | ~28 ms | ~20 ms | ~10% |
| **C#/.NET** | ~27 ms | ~19 ms | ~6% |
| **Node.js** | ~29 ms | ~21 ms | ~12% |

*All measurements on Windows 11, Intel Core i7*

---

## 🔗 Common Features

All language bindings support:

- ✅ **In-memory mode** - Fast temporary storage
- ✅ **File-based persistence** - Durable storage
- ✅ **CRUD operations** - Insert, Get, Delete
- ✅ **Batch operations** - High-performance bulk inserts
- ✅ **Transaction support** - ACID guarantees
- ✅ **Error handling** - Language-idiomatic error reporting

---

## 🚀 Getting Started

1. **Choose your language** from the options above
2. **Follow the installation** instructions
3. **Check the examples** in each `lang/<language>/examples/` directory
4. **Read the API reference** in each language's README

---

## 📦 Package Availability

| Language | Package Manager | Status |
|----------|----------------|--------|
| Python | PyPI | 🚧 Coming Soon |
| C#/.NET | NuGet | 🚧 Coming Soon |
| Node.js | npm | 🚧 Coming Soon |
| C/C++ | Manual Build | ✅ Available |

---

## 🤝 Contributing

Want to add bindings for another language? See our [Contributing Guide](https://github.com/bytelogiccore-spec/ByteRAG/blob/main/docs/english/contributing.md).

---

## 📄 License

All language bindings are dual-licensed under MIT or Apache-2.0, same as the core ByteRAG library.


