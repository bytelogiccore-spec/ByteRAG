//! Database Traits — 기능별 역할 분리
//!
//! Database 구조체의 메서드를 기능별 Trait으로 분리하여
//! 명확한 책임 구분과 테스트 용이성을 제공합니다.

use crate::error::ByteRagResult;

// ════════════════════════════════════════════
// Core CRUD Operations
// ════════════════════════════════════════════

/// 핵심 CRUD 작업을 제공하는 Trait
pub trait DatabaseCore {
    /// 데이터 삽입
    fn insert(&self, table: &str, key: &[u8], value: &[u8]) -> ByteRagResult<()>;

    /// 데이터 조회
    fn get(&self, table: &str, key: &[u8]) -> ByteRagResult<Option<Vec<u8>>>;

    /// 데이터 삭제
    fn delete(&self, table: &str, key: &[u8]) -> ByteRagResult<()>;

    /// 전체 스캔
    fn scan(&self, table: &str) -> ByteRagResult<Vec<(Vec<u8>, Vec<u8>)>>;

    /// 메모리 → 디스크 플러시
    fn flush(&self) -> ByteRagResult<()>;

    /// 배치 삽입
    fn insert_batch(&self, table: &str, entries: Vec<(Vec<u8>, Vec<u8>)>) -> ByteRagResult<()>;

    /// 값이 없을 때만 삽입 (Atomic CAS)
    fn insert_if_not_exists(&self, table: &str, key: &[u8], value: &[u8]) -> ByteRagResult<bool>;

    /// 기존 값과 비교하여 일치할 때만 새로운 값으로 교체 (Atomic CAS)
    fn compare_and_swap(
        &self,
        table: &str,
        key: &[u8],
        expected: &[u8],
        new_value: &[u8],
    ) -> ByteRagResult<bool>;

    /// 기존 값이 존재할 때만 업데이트 (Atomic CAS)
    fn update_if_exists(&self, table: &str, key: &[u8], value: &[u8]) -> ByteRagResult<bool>;

    /// 기존 값과 일치할 때만 삭제 (Atomic CAS)
    fn delete_if_equals(&self, table: &str, key: &[u8], expected: &[u8]) -> ByteRagResult<bool>;
}

// ════════════════════════════════════════════
// SQL Execution
// ════════════════════════════════════════════

/// SQL 실행 기능을 제공하는 Trait
pub trait DatabaseSql {
    /// SQL 문 실행 (RecordBatch는 engine에서 정의)
    fn execute_sql(&self, sql: &str) -> ByteRagResult<Vec<arrow::record_batch::RecordBatch>>;

    /// 테이블 등록 (Arrow RecordBatch)
    fn register_table(&self, name: &str, batches: Vec<arrow::record_batch::RecordBatch>);

    /// 배치 추가
    fn append_batch(
        &self,
        table: &str,
        batch: arrow::record_batch::RecordBatch,
    ) -> ByteRagResult<()>;
}

// ════════════════════════════════════════════
// Query Builder
// ════════════════════════════════════════════

/// Fluent 스타일 쿼리 빌더를 제공하는 Trait
/// (구체적인 타입은 api/query.rs에서 정의)
pub trait DatabaseQuery {
    // Query Builder 메서드는 engine/database.rs에서 구현
}

// ════════════════════════════════════════════
// Transaction Management
// ════════════════════════════════════════════

/// 트랜잭션 관리 기능을 제공하는 Trait
pub trait DatabaseTransaction {
    /// 트랜잭션 시작
    fn begin(
        &self,
    ) -> ByteRagResult<crate::transaction::api::Transaction<'_, crate::transaction::api::Active>>;
}

// ════════════════════════════════════════════
// Snapshot & Backup
// ════════════════════════════════════════════

/// 스냅샷 및 백업 기능을 제공하는 Trait
pub trait DatabaseSnapshot {
    /// 데이터베이스를 파일로 저장
    fn save_to_file(&self, path: &str) -> ByteRagResult<()>;

    /// 파일에서 데이터베이스 로드
    fn load_from_file(path: &str) -> ByteRagResult<Self>
    where
        Self: Sized;
}

// ════════════════════════════════════════════
// Native Serde Support
// ════════════════════════════════════════════

/// `serde` 기반의 구조체 직접 입출력을 지원하는 Trait
pub trait DatabaseSerde {
    /// 구조체를 직렬화하여 삽입
    fn insert_struct<T: serde::Serialize>(
        &self,
        table: &str,
        key: &[u8],
        data: &T,
    ) -> ByteRagResult<()>;

    /// 데이터를 조회하여 구조체로 역직렬화
    fn get_struct<T: serde::de::DeserializeOwned>(
        &self,
        table: &str,
        key: &[u8],
    ) -> ByteRagResult<Option<T>>;
}
