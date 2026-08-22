---
layout: default
title: Installation
parent: Python (byterag-py)
grand_parent: Packages
great_grand_parent: English
nav_order: 1
---

# Installation

## Install from PyPI

```bash
pip install byterag-py
```

## Install with pip

```bash
python -m pip install byterag-py
```

## Install with uv (Recommended)

```bash
uv pip install byterag-py
```

## Requirements

- **Python**: 3.8 or higher
- **Platform**: Windows x64 (currently tested)
  - Linux x64: Planned
  - macOS (Intel/Apple Silicon): Planned

## Verify Installation

```python
from byterag_py import Database

db = Database.open_in_memory()
print("ByteRAG Python loaded successfully!")
db.close()
```

## Virtual Environment

### venv

```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
venv\Scripts\activate     # Windows

pip install byterag-py
```

### conda

```bash
conda create -n ByteRAG python=3.11
conda activate ByteRAG
pip install byterag-py
```

## Troubleshooting

### Module Not Found

**Cause**: Installation failed or wrong Python environment

**Solution**:
```bash
# Verify installation
pip list | grep byterag-py

# Reinstall
pip uninstall byterag-py
pip install byterag-py
```

### Import Error on Windows

**Cause**: Missing Visual C++ Redistributable

**Solution**:
1. Download [Microsoft Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
2. Install and restart

### Version Check

```bash
pip show byterag-py
```

## Next Steps

- [Quick Start](quickstart) - Get started in 5 minutes
- [SQL Guide](sql-guide) - SQL usage
- [API Reference](api-reference) - Complete API


