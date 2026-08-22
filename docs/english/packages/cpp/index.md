---
layout: default
title: C/C++ (byterag-ffi)
nav_order: 5
parent: Packages
grand_parent: English
has_children: true
---

# C/C++ — byterag-ffi

Official C/C++ FFI (Foreign Function Interface) bindings for ByteRAG high-performance embedded database.

## Key Features

- 🚀 **Native Performance**: Direct Rust core calls
- 💾 **5-Tier Storage**: WOS → L0 → L1 → L2 → Cold Storage
- 🔒 **MVCC Transactions**: Snapshot isolation support
- 📊 **SQL Support**: Full DDL + DML support
- 🔐 **Encryption**: AES-GCM-SIV, ChaCha20-Poly1305
- 🔧 **C89 Compatible**: All C/C++ compilers supported

## Quick Start

### C Example

```c
#include "ByteRAG.h"
#include <stdio.h>

int main() {
    // Open database
    DbxDatabase* db = byterag_open_in_memory();
    
    // KV operations
    const char* key = "user:1";
    const char* value = "Alice";
    byterag_insert(db, "users", (uint8_t*)key, strlen(key), (uint8_t*)value, strlen(value));
    
    // Query
    uint8_t* result = NULL;
    size_t result_len = 0;
    byterag_get(db, "users", (uint8_t*)key, strlen(key), &result, &result_len);
    
    if (result) {
        printf("Value: %.*s\n", (int)result_len, result);
        byterag_free_bytes(result);
    }
    
    // SQL operations
    byterag_execute_sql(db, "CREATE TABLE users (id INTEGER, name TEXT)");
    byterag_execute_sql(db, "INSERT INTO users VALUES (1, 'Alice')");
    
    char* sql_result = byterag_execute_sql(db, "SELECT * FROM users");
    printf("SQL Result: %s\n", sql_result);
    byterag_free_string(sql_result);
    
    // Cleanup
    byterag_close(db);
    return 0;
}
```

### C++ Example

```cpp
#include "ByteRAG.hpp"
#include <iostream>
#include <string>

int main() {
    // RAII wrapper
    ByteRAG::Database db = ByteRAG::Database::openInMemory();
    
    // KV operations
    db.insert("users", "user:1", "Alice");
    
    auto value = db.get("users", "user:1");
    if (value) {
        std::cout << "Value: " << *value << std::endl;
    }
    
    // SQL operations
    db.executeSql("CREATE TABLE users (id INTEGER, name TEXT)");
    db.executeSql("INSERT INTO users VALUES (1, 'Alice')");
    
    auto result = db.executeSql("SELECT * FROM users");
    std::cout << "SQL Result: " << result << std::endl;
    
    return 0;
}
```

## Documentation

- [Installation](installation) - Headers and library setup
- [Quick Start](quickstart) - Get started in 5 minutes
- [C API](c-api) - C function reference
- [C++ API](cpp-api) - C++ class reference
- [KV Operations](kv-operations) - Key-Value operations guide
- [SQL Guide](sql-guide) - SQL usage
- [Advanced](advanced) - Transactions, encryption, multithreading
- [Examples](examples) - Real-world examples

## Version Info

- **Current Version**: {{ site.byterag_version }}
- **C Standard**: C89+
- **C++ Standard**: C++11+ (C++ wrapper)
- **Platform**: Windows x64 (Linux/macOS planned)

## License

MIT License


