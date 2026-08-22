---
layout: default
title: 예제
nav_order: 10
parent: 한국어
description: "ByteRAG 코드 예제"
---

# 예제

ByteRAG의 기능을 보여주는 실용적인 코드 예제들입니다.

## 🚀 시작하기

ByteRAG를 처음 접하신다면 여기부터 시작하세요:

- [**빠른 시작**](./examples/quick-start.md) - 5분 시작 가이드 (CRUD 기본)
- [**SQL 빠른 시작**](./examples/sql-quick-start.md) - SQL 기본 사용법

## 🔒 보안 및 데이터 보호

암호화와 압축으로 데이터를 보호하세요:

- [**암호화**](./examples/encryption.md) - AES-256-GCM-SIV 및 ChaCha20-Poly1305 암호화
- [**압축**](./examples/compression.md) - 공간 절약을 위한 ZSTD 압축

## ⚡ 성능 최적화

다음 기능들을 사용하여 성능을 극대화하세요:

- [**인덱싱**](./examples/indexing.md) - 빠른 조회를 위한 Bloom Filter 인덱스

## 🔄 안정성

데이터의 영속성을 보장하세요:

- [**WAL 복구**](./examples/wal-recovery.md) - 크래시 복구를 위한 Write-Ahead Log

## 📚 예제 카테고리

### 난이도별

| 수준 | 예제 |
|-------|----------|
| **기초** | [빠른 시작](./examples/quick-start.md), [SQL 빠른 시작](./examples/sql-quick-start.md) |
| **중급** | [인덱싱](./examples/indexing.md), [암호화](./examples/encryption.md) |
| **고급** | [압축](./examples/compression.md), [WAL 복구](./examples/wal-recovery.md) |

### 기능별

| 기능 | 예제 |
|---------|----------|
| **저장소** | [빠른 시작](./examples/quick-start.md), [압축](./examples/compression.md) |
| **쿼리** | [SQL 빠른 시작](./examples/sql-quick-start.md), [인덱싱](./examples/indexing.md) |
| **안정성** | [WAL 복구](./examples/wal-recovery.md) |
| **보안** | [암호화](./examples/encryption.md) |

## 🎯 빠른 탐색

**다음 작업을 하고 싶습니다...**

- **데이터 저장 및 조회** → [빠른 시작](./examples/quick-start.md)
- **SQL 쿼리 실행** → [SQL 빠른 시작](./examples/sql-quick-start.md)
- **중요 데이터 보호** → [암호화](./examples/encryption.md)
- **조회 속도 향상** → [인덱싱](./examples/indexing.md)
- **디스크 사용량 절차** → [압축](./examples/compression.md)
- **영속성 보장** → [WAL 복구](./examples/wal-recovery.md)

## 💻 예제 실행하기

모든 예제는 `core/byterag-core/examples/` 폴더에 있으며, 다음 명령어로 실행할 수 있습니다:

```bash
# 모든 예제 목록 확인
cargo run --example

# 특정 예제 실행
cargo run --example encryption
cargo run --example transactions
cargo run --example gpu_acceleration
```

## 📖 문서 구조

각 예제는 다음 내용을 포함합니다:

- **개요 (Overview)**: 해당 기능에 대한 설명
- **빠른 시작 (Quick Start)**: 시작을 위한 최소한의 코드
- **단계별 가이드 (Step-by-Step Guide)**: 상세 안내
- **전체 예제 (Complete Example)**: 실행 가능한 전체 코드
- **성능 팁 (Performance Tips)**: 최적화 제안
- **다음 단계 (Next Steps)**: 관련 예제 및 기능

## 🔗 관련 리소스

- [아키텍처](../architecture.md) - ByteRAG의 5-Tier 하이브리드 스토리지 이해하기
- [벤치마크](../benchmarks.md) - 성능 비교 확인
- [API 레퍼런스](../api/) - 상세 API 문서

---

**도움이 필요하신가요?** [이슈를 등록](https://github.com/bytelogiccore-spec/ByteRAG/issues)해 주세요.



