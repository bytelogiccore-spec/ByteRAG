---
layout: default
title: .NET (DBX.Dotnet)
nav_order: 2
parent: 패키지
grand_parent: 한국어
has_children: true
---

# .NET — DBX.Dotnet

[![NuGet](https://img.shields.io/nuget/v/DBX.Dotnet.svg)](https://www.nuget.org/packages/DBX.Dotnet/)

고성능 임베디드 데이터베이스 ByteRAG의 공식 .NET 바인딩입니다.

## 주요 기능

- 🚀 **네이티브 성능**: Rust 기반 P/Invoke
- 💾 **5-Tier 스토리지**: WOS → L0 → L1 → L2 → Cold Storage
- 🔒 **MVCC 트랜잭션**: 스냅샷 격리 지원
- 📊 **SQL 지원**: DDL + DML 완벽 지원
- 🔐 **암호화**: AES-GCM-SIV, ChaCha20-Poly1305
- 🎯 **.NET Standard 2.0**: .NET Framework, .NET Core, .NET 5+ 모두 지원

## 빠른 시작

```bash
dotnet add package DBX.Dotnet
```

```csharp
using DBX.Dotnet;

using (var db = Database.OpenInMemory())
{
    // KV 작업
    db.Insert("users", "user:1"u8.ToArray(), "Alice"u8.ToArray());
    var value = db.Get("users", "user:1"u8.ToArray());
    Console.WriteLine(Encoding.UTF8.GetString(value));  // Alice
    
    // SQL 작업
    db.ExecuteSql("CREATE TABLE users (id INTEGER, name TEXT)");
    db.ExecuteSql("INSERT INTO users VALUES (1, 'Alice')");
    var result = db.ExecuteSql("SELECT * FROM users");
    Console.WriteLine(result);
}
```

## 문서 구조

- [설치](installation) - 설치 및 환경 설정
- [빠른 시작](quickstart) - 5분 안에 시작하기
- [KV 작업](kv-operations) - Key-Value 작업 가이드
- [SQL 가이드](sql-guide) - SQL 사용법
- [고급 기능](advanced) - 트랜잭션, 암호화, 성능 튜닝
- [API 레퍼런스](api-reference) - 전체 API 문서
- [실전 예제](examples) - 실무 활용 예제

## 버전 정보

- **현재 버전**: {{ site.byterag_version }}
- **.NET 요구사항**: .NET Standard 2.0+ (.NET Framework 4.6.1+, .NET Core 2.0+, .NET 5+)
- **플랫폼**: Windows x64 (Linux/macOS 계획됨)

## 라이선스

MIT License


