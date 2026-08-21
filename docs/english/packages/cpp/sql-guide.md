---
layout: default
title: SQL Guide
parent: C/C++ (byterag-ffi)
grand_parent: Packages
great_grand_parent: English
nav_order: 5
---

# SQL Guide

Complete SQL guide for C/C++.

## CREATE TABLE

```c
byterag_execute_sql(db, "CREATE TABLE users (id INTEGER, name TEXT)");
```

## INSERT

```c
byterag_execute_sql(db, "INSERT INTO users VALUES (1, 'Alice')");
```

## SELECT

```c
char* result = byterag_execute_sql(db, "SELECT * FROM users");
printf("%s\n", result);
byterag_free_string(result);
```

## UPDATE

```c
byterag_execute_sql(db, "UPDATE users SET name = 'Bob' WHERE id = 1");
```

## DELETE

```c
byterag_execute_sql(db, "DELETE FROM users WHERE id = 1");
```

## Transactions

```c
DbxTransaction* tx = byterag_begin_transaction(db);
byterag_execute_sql(db, "INSERT INTO users VALUES (1, 'Alice')");
byterag_commit(tx);
```

## Next Steps

- [KV Operations](kv-operations) - Key-Value operations
- [C API](c-api) - C function reference

