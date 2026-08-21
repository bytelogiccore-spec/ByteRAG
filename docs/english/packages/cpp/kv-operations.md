---
layout: default
title: KV Operations
parent: C/C++ (byterag-ffi)
grand_parent: Packages
great_grand_parent: English
nav_order: 6
---

# Key-Value Operations

High-performance KV operations for C/C++.

## Basic CRUD (C)

```c
// Insert
const char* key = "user:1";
const char* value = "Alice";
byterag_insert(db, "users", (uint8_t*)key, strlen(key), (uint8_t*)value, strlen(value));

// Get
uint8_t* result = NULL;
size_t result_len = 0;
byterag_get(db, "users", (uint8_t*)key, strlen(key), &result, &result_len);
if (result) {
    printf("%.*s\n", (int)result_len, result);
    byterag_free_bytes(result);
}

// Delete
byterag_delete(db, "users", (uint8_t*)key, strlen(key));
```

## Basic CRUD (C++)

```cpp
// Insert
db.insert("users", "user:1", "Alice");

// Get
auto value = db.get("users", "user:1");
if (value) {
    std::cout << *value << std::endl;
}

// Delete
db.remove("users", "user:1");
```

## Batch Operations

```c
for (int i = 0; i < 10000; i++) {
    char key[32], value[64];
    snprintf(key, sizeof(key), "key:%d", i);
    snprintf(value, sizeof(value), "value:%d", i);
    byterag_insert(db, "data", (uint8_t*)key, strlen(key), (uint8_t*)value, strlen(value));
}
byterag_flush(db);
```

## Next Steps

- [SQL Guide](sql-guide) - SQL usage
- [Advanced](advanced) - Transactions, multithreading

