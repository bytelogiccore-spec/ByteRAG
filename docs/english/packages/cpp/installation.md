---
layout: default
title: Installation
parent: C/C++ (byterag-ffi)
grand_parent: Packages
great_grand_parent: English
nav_order: 1
---

# Installation

## Download

Download the latest release from [GitHub Releases](https://github.com/bytelogiccore-spec/DBX/releases).

## Package Contents

```
byterag-ffi/
├── include/
│   ├── dbx.h        # C API header
│   └── dbx.hpp      # C++ wrapper header
├── lib/
│   └── byterag_ffi.dll  # Windows x64
└── README.md
```

## Visual Studio Setup

### 1. Add Include Directory

Project Properties → C/C++ → General → Additional Include Directories:
```
D:\path\to\byterag-ffi\include
```

### 2. Add Library Directory

Project Properties → Linker → General → Additional Library Directories:
```
D:\path\to\byterag-ffi\lib
```

### 3. Link Library

Project Properties → Linker → Input → Additional Dependencies:
```
byterag_ffi.lib
```

### 4. Copy DLL

Copy `byterag_ffi.dll` to your output directory.

## GCC/MinGW Setup

```bash
gcc -I./include -L./lib main.c -lbyterag_ffi -o myapp.exe
```

## CMake Setup

```cmake
cmake_minimum_required(VERSION 3.10)
project(MyApp)

include_directories(${CMAKE_SOURCE_DIR}/byterag-ffi/include)
link_directories(${CMAKE_SOURCE_DIR}/byterag-ffi/lib)

add_executable(myapp main.c)
target_link_libraries(myapp byterag_ffi)
```

## Verify Installation

### C

```c
#include "dbx.h"
#include <stdio.h>

int main() {
    DbxDatabase* db = byterag_open_in_memory();
    printf("DBX C loaded successfully!\n");
    byterag_close(db);
    return 0;
}
```

### C++

```cpp
#include "dbx.hpp"
#include <iostream>

int main() {
    auto db = dbx::Database::openInMemory();
    std::cout << "DBX C++ loaded successfully!" << std::endl;
    return 0;
}
```

## Next Steps

- [Quick Start](quickstart) - Get started in 5 minutes
- [C API](c-api) - C function reference
- [C++ API](cpp-api) - C++ class reference

