---
layout: default
title: C API
parent: C/C++ (byterag-ffi)
grand_parent: Packages
great_grand_parent: English
nav_order: 3
---

# C API Reference

Complete C API reference for ByteRAG.

## Database Functions

### `DbxDatabase* byterag_open(const char* path)`

Opens a file-based database.

**Example:**
```c
DbxDatabase* db = byterag_open("mydb.db");
```

### `DbxDatabase* byterag_open_in_memory(void)`

Opens an in-memory database.

**Example:**
```c
DbxDatabase* db = byterag_open_in_memory();
```

### `void byterag_close(DbxDatabase* db)`

Closes the database.

**Example:**
```c
byterag_close(db);
```

## Key-Value Functions

### `int byterag_insert(DbxDatabase* db, const char* table, const uint8_t* key, size_t key_len, const uint8_t* value, size_t value_len)`

Inserts a key-value pair.

**Returns:** 0 on success, -1 on error

**Example:**
```c
const char* key = "user:1";
const char* value = "Alice";
byterag_insert(db, "users", (uint8_t*)key, strlen(key), (uint8_t*)value, strlen(value));
```

### `int byterag_get(DbxDatabase* db, const char* table, const uint8_t* key, size_t key_len, uint8_t** value_out, size_t* value_len_out)`

Gets a value by key.

**Returns:** 0 on success, -1 on error

**Example:**
```c
uint8_t* result = NULL;
size_t result_len = 0;
if (byterag_get(db, "users", (uint8_t*)key, strlen(key), &result, &result_len) == 0) {
    printf("%.*s\n", (int)result_len, result);
    byterag_free_bytes(result);
}
```

### `int byterag_delete(DbxDatabase* db, const char* table, const uint8_t* key, size_t key_len)`

Deletes a key.

**Returns:** 0 on success, -1 on error

**Example:**
```c
byterag_delete(db, "users", (uint8_t*)key, strlen(key));
```

### `size_t byterag_count(DbxDatabase* db, const char* table)`

Returns the number of rows.

**Example:**
```c
size_t count = byterag_count(db, "users");
printf("Total: %zu\n", count);
```

## SQL Functions

### `char* byterag_execute_sql(DbxDatabase* db, const char* sql)`

Executes a SQL statement.

**Returns:** Result string (must be freed with `byterag_free_string`)

**Example:**
```c
char* result = byterag_execute_sql(db, "SELECT * FROM users");
printf("%s\n", result);
byterag_free_string(result);
```

## Transaction Functions

### `DbxTransaction* byterag_begin_transaction(DbxDatabase* db)`

Begins a transaction.

**Example:**
```c
DbxTransaction* tx = byterag_begin_transaction(db);
```

### `int byterag_commit(DbxTransaction* tx)`

Commits the transaction.

**Returns:** 0 on success, -1 on error

**Example:**
```c
byterag_commit(tx);
```

### `int byterag_rollback(DbxTransaction* tx)`

Rolls back the transaction.

**Returns:** 0 on success, -1 on error

**Example:**
```c
byterag_rollback(tx);
```

## Utility Functions

### `void byterag_flush(DbxDatabase* db)`

Flushes the buffer to disk.

**Example:**
```c
byterag_flush(db);
```

### `void byterag_free_bytes(uint8_t* ptr)`

Frees memory allocated by ByteRAG.

**Example:**
```c
byterag_free_bytes(result);
```

### `void byterag_free_string(char* ptr)`

Frees a string allocated by ByteRAG.

**Example:**
```c
byterag_free_string(sql_result);
```

## Error Handling

All functions return 0 on success and -1 on error. Check return values:

```c
if (byterag_insert(db, "users", key, key_len, value, value_len) != 0) {
    fprintf(stderr, "Insert failed\n");
    return 1;
}
```

## Complete Example

```c
#include "ByteRAG.h"
#include <stdio.h>
#include <string.h>

int main() {
    // Open database
    DbxDatabase* db = byterag_open("example.db");
    if (!db) {
        fprintf(stderr, "Failed to open database\n");
        return 1;
    }
    
    // Begin transaction
    DbxTransaction* tx = byterag_begin_transaction(db);
    
    // Insert KV
    const char* key = "user:1";
    const char* value = "Alice";
    byterag_insert(db, "users", (uint8_t*)key, strlen(key), (uint8_t*)value, strlen(value));
    
    // Commit
    byterag_commit(tx);
    
    // Query
    uint8_t* result = NULL;
    size_t result_len = 0;
    if (byterag_get(db, "users", (uint8_t*)key, strlen(key), &result, &result_len) == 0) {
        printf("Value: %.*s\n", (int)result_len, result);
        byterag_free_bytes(result);
    }
    
    // SQL
    byterag_execute_sql(db, "CREATE TABLE products (id INTEGER, name TEXT)");
    byterag_execute_sql(db, "INSERT INTO products VALUES (1, 'Laptop')");
    
    char* sql_result = byterag_execute_sql(db, "SELECT * FROM products");
    printf("SQL Result: %s\n", sql_result);
    byterag_free_string(sql_result);
    
    // Cleanup
    byterag_flush(db);
    byterag_close(db);
    
    return 0;
}
```

## Next Steps

- [C++ API](cpp-api) - C++ wrapper
- [SQL Guide](sql-guide) - SQL usage
- [Examples](examples) - More examples


