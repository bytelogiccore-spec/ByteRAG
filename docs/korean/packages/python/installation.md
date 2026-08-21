---
layout: default
title: 설치
parent: Python (byterag-py)
grand_parent: 패키지
great_grand_parent: 한국어
nav_order: 1
---

# 설치

## PyPI에서 설치

```bash
pip install byterag-py
```

## 소스에서 빌드

```bash
git clone https://github.com/bytelogiccore-spec/DBX.git
cd DBX/lang/python
pip install maturin
maturin develop --release
```

## 요구사항

- **Python**: 3.8 이상
- **플랫폼**: Windows x64 (현재 테스트 완료)
  - Linux x64: 계획됨
  - macOS (Intel/Apple Silicon): 계획됨

## 설치 확인

```python
import byterag_py
print(byterag_py.__version__)  # {{ site.byterag_py_version }}
```

## 가상 환경 (권장)

```bash
# venv 생성
python -m venv venv

# 활성화 (Windows)
venv\Scripts\activate

# 활성화 (Linux/macOS)
source venv/bin/activate

# byterag-py 설치
pip install byterag-py
```

## 문제 해결

### ImportError: DLL load failed

**원인**: Visual C++ Redistributable 누락

**해결**:
1. [Microsoft Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) 다운로드
2. 설치 후 Python 재시작

### pip install 실패

```bash
# pip 업그레이드
python -m pip install --upgrade pip

# 재시도
pip install byterag-py
```

### 특정 버전 설치

```bash
# 최신 베타
pip install byterag-py --pre

# 특정 버전
pip install byterag-py=={{ site.byterag_py_version }}
```

