---
layout: default
title: C API
parent: C/C++ (byterag-ffi)
grand_parent: 패키지
great_grand_parent: 한국어
nav_order: 3
---

# C API 레퍼런스

## 타입 정의

### DbxDatabase

```c
typedef struct DbxDatabase DbxDatabase;
```

불투명 포인터. 데이터베이스 핸들을 나타냅니다.

### DbxTransaction

```c
typedef struct DbxTransaction DbxTransaction;
```

불투명 포인터. 트랜잭션 핸들을 나타냅니다.

## 데이터베이스 관리

### byterag_open

```c
DbxDatabase* byterag_open(const char* path);
```

파일 기반 데이터베이스를 엽니다.

**매개변수:**
- `path`: 데이터베이스 파일 경로 (NULL 종료 문자열)

**반환:** 데이터베이스 핸들 또는 NULL (실패 시)

**예제:**
```c
DbxDatabase* db = byterag_open("mydb.db");
if (!db) {
    fprintf(stderr, "Failed to open database\n");
    return 1;
}
```

### byterag_open_in_memory

```c
DbxDatabase* byterag_open_in_memory(void);
```

인메모리 데이터베이스를 엽니다.

**반환:** 데이터베이스 핸들 또는 NULL (실패 시)

**예제:**
```c
DbxDatabase* db = byterag_open_in_memory();
```

### byterag_close

```c
void byterag_close(DbxDatabase* db);
```

데이터베이스를 닫고 리소스를 해제합니다.

**매개변수:**
- `db`: 데이터베이스 핸들

**예제:**
```c
byterag_close(db);
```

## Key-Value 작업

### byterag_insert

```c
int byterag_insert(
    DbxDatabase* db,
    const char* table,
    const uint8_t* key,
    size_t key_len,
    const uint8_t* value,
    size_t value_len
);
```

키-값 쌍을 삽입합니다.

**매개변수:**
- `db`: 데이터베이스 핸들
- `table`: 테이블 이름
- `key`: 키 (바이트 배열)
- `key_len`: 키 길이
- `value`: 값 (바이트 배열)
- `value_len`: 값 길이

**반환:** 0 (성공), -1 (실패)

**예제:**
```c
const char* key = "user:1";
const char* value = "Alice";
int result = byterag_insert(db, "users", 
    (uint8_t*)key, strlen(key),
    (uint8_t*)value, strlen(value));
```

### byterag_get

```c
int byterag_get(
    DbxDatabase* db,
    const char* table,
    const uint8_t* key,
    size_t key_len,
    uint8_t** value_out,
    size_t* value_len_out
);
```

키로 값을 조회합니다.

**매개변수:**
- `db`: 데이터베이스 핸들
- `table`: 테이블 이름
- `key`: 키 (바이트 배열)
- `key_len`: 키 길이
- `value_out`: 값 출력 포인터 (호출자가 `byterag_free_bytes`로 해제 필요)
- `value_len_out`: 값 길이 출력 포인터

**반환:** 0 (성공), -1 (실패 또는 키 없음)

**예제:**
```c
uint8_t* value = NULL;
size_t value_len = 0;
if (byterag_get(db, "users", (uint8_t*)key, strlen(key), &value, &value_len) == 0) {
    printf("Value: %.*s\n", (int)value_len, value);
    byterag_free_bytes(value);
}
```

### byterag_delete

```c
int byterag_delete(
    DbxDatabase* db,
    const char* table,
    const uint8_t* key,
    size_t key_len
);
```

키를 삭제합니다.

**매개변수:**
- `db`: 데이터베이스 핸들
- `table`: 테이블 이름
- `key`: 키 (바이트 배열)
- `key_len`: 키 길이

**반환:** 0 (성공), -1 (실패)

**예제:**
```c
byterag_delete(db, "users", (uint8_t*)key, strlen(key));
```

### byterag_count

```c
size_t byterag_count(DbxDatabase* db, const char* table);
```

테이블의 행 개수를 반환합니다.

**매개변수:**
- `db`: 데이터베이스 핸들
- `table`: 테이블 이름

**반환:** 행 개수

**예제:**
```c
size_t count = byterag_count(db, "users");
printf("Total users: %zu\n", count);
```

## SQL 작업

### byterag_execute_sql

```c
char* byterag_execute_sql(DbxDatabase* db, const char* sql);
```

SQL 문을 실행합니다.

**매개변수:**
- `db`: 데이터베이스 핸들
- `sql`: SQL 문 (NULL 종료 문자열)

**반환:** 결과 문자열 (JSON 형식, 호출자가 `byterag_free_string`으로 해제 필요) 또는 NULL (실패 시)

**예제:**
```c
// DDL
byterag_execute_sql(db, "CREATE TABLE users (id INTEGER, name TEXT)");

// DML
byterag_execute_sql(db, "INSERT INTO users VALUES (1, 'Alice')");

// 조회
char* result = byterag_execute_sql(db, "SELECT * FROM users");
if (result) {
    printf("Result: %s\n", result);
    byterag_free_string(result);
}
```

## 트랜잭션

### byterag_begin_transaction

```c
DbxTransaction* byterag_begin_transaction(DbxDatabase* db);
```

트랜잭션을 시작합니다.

**매개변수:**
- `db`: 데이터베이스 핸들

**반환:** 트랜잭션 핸들 또는 NULL (실패 시)

**예제:**
```c
DbxTransaction* tx = byterag_begin_transaction(db);
```

### byterag_commit

```c
int byterag_commit(DbxTransaction* tx);
```

트랜잭션을 커밋합니다.

**매개변수:**
- `tx`: 트랜잭션 핸들

**반환:** 0 (성공), -1 (실패)

**예제:**
```c
byterag_commit(tx);
```

### byterag_rollback

```c
int byterag_rollback(DbxTransaction* tx);
```

트랜잭션을 롤백합니다.

**매개변수:**
- `tx`: 트랜잭션 핸들

**반환:** 0 (성공), -1 (실패)

**예제:**
```c
byterag_rollback(tx);
```

## 유틸리티

### byterag_flush

```c
void byterag_flush(DbxDatabase* db);
```

버퍼를 디스크에 플러시합니다.

**매개변수:**
- `db`: 데이터베이스 핸들

**예제:**
```c
byterag_flush(db);
```

### byterag_free_bytes

```c
void byterag_free_bytes(uint8_t* ptr);
```

`byterag_get`으로 할당된 바이트 배열을 해제합니다.

**매개변수:**
- `ptr`: 해제할 포인터

**예제:**
```c
byterag_free_bytes(value);
```

### byterag_free_string

```c
void byterag_free_string(char* ptr);
```

`byterag_execute_sql`로 할당된 문자열을 해제합니다.

**매개변수:**
- `ptr`: 해제할 포인터

**예제:**
```c
byterag_free_string(result);
```

## 에러 처리

### byterag_last_error

```c
const char* byterag_last_error(void);
```

마지막 에러 메시지를 반환합니다.

**반환:** 에러 메시지 (NULL 종료 문자열) 또는 NULL (에러 없음)

**예제:**
```c
if (byterag_insert(db, "users", key, key_len, value, value_len) != 0) {
    const char* error = byterag_last_error();
    fprintf(stderr, "Error: %s\n", error ? error : "Unknown");
}
```

## 완전한 예제

```c
#include "dbx.h"
#include <stdio.h>
#include <string.h>

int main() {
    // 데이터베이스 열기
    DbxDatabase* db = byterag_open("example.db");
    if (!db) {
        fprintf(stderr, "Failed to open database: %s\n", byterag_last_error());
        return 1;
    }

    // 트랜잭션 시작
    DbxTransaction* tx = byterag_begin_transaction(db);
    if (!tx) {
        fprintf(stderr, "Failed to begin transaction\n");
        byterag_close(db);
        return 1;
    }

    // KV 삽입
    const char* key = "user:1";
    const char* value = "Alice";
    if (byterag_insert(db, "users", (uint8_t*)key, strlen(key), 
                   (uint8_t*)value, strlen(value)) != 0) {
        fprintf(stderr, "Insert failed: %s\n", byterag_last_error());
        byterag_rollback(tx);
        byterag_close(db);
        return 1;
    }

    // 커밋
    if (byterag_commit(tx) != 0) {
        fprintf(stderr, "Commit failed\n");
        byterag_close(db);
        return 1;
    }

    // 조회
    uint8_t* result = NULL;
    size_t result_len = 0;
    if (byterag_get(db, "users", (uint8_t*)key, strlen(key), &result, &result_len) == 0) {
        printf("Value: %.*s\n", (int)result_len, result);
        byterag_free_bytes(result);
    }

    // SQL 실행
    char* sql_result = byterag_execute_sql(db, "SELECT COUNT(*) FROM users");
    if (sql_result) {
        printf("SQL Result: %s\n", sql_result);
        byterag_free_string(sql_result);
    }

    // 정리
    byterag_flush(db);
    byterag_close(db);
    return 0;
}
```

## 다음 단계

- [C++ API](cpp-api) - C++ 래퍼 사용
- [SQL 가이드](sql-guide) - SQL 사용법
- [실전 예제](examples) - 더 많은 예제

