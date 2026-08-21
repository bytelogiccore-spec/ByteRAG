"""DBX Database wrapper for Python (ctypes FFI)"""

import ctypes
import platform
from pathlib import Path
from typing import Optional, List, Tuple


class Database:
    """DBX Database wrapper
    
    Example:
        >>> db = Database("./my_db")
        >>> db.insert("users", b"user:1", b"Alice")
        >>> value = db.get("users", b"user:1")
        >>> print(value)
        b'Alice'
    """
    
    def __init__(self, path: str):
        """Open a database at the given path"""
        self._lib = self._load_library()
        self._setup_functions()
        
        path_bytes = path.encode('utf-8')
        self._handle = self._lib.byterag_open(path_bytes)
        
        if not self._handle:
            raise RuntimeError(f"Failed to open database at {path}")
    
    @classmethod
    def open_in_memory(cls) -> 'Database':
        """Open an in-memory database"""
        db = cls.__new__(cls)
        db._lib = db._load_library()
        db._setup_functions()
        db._handle = db._lib.byterag_open_in_memory()
        
        if not db._handle:
            raise RuntimeError("Failed to open in-memory database")
        
        return db

    @classmethod
    def load_from_file(cls, path: str) -> 'Database':
        """Load a database from a snapshot file"""
        db = cls.__new__(cls)
        db._lib = db._load_library()
        db._setup_functions()
        db._handle = db._lib.byterag_load_from_file(path.encode('utf-8'))

        if not db._handle:
            raise RuntimeError(f"Failed to load database from {path}")

        return db

    # ═══════════════════════════════════════════════════
    # CRUD Operations
    # ═══════════════════════════════════════════════════
    
    def insert(self, table: str, key: bytes, value: bytes) -> None:
        """Insert a key-value pair into a table"""
        table_bytes = table.encode('utf-8')
        key_array = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
        value_array = (ctypes.c_uint8 * len(value)).from_buffer_copy(value)
        
        result = self._lib.byterag_insert(
            self._handle, table_bytes,
            key_array, len(key),
            value_array, len(value)
        )
        
        if result != 0:
            raise RuntimeError(f"Insert failed with error code: {result}")
    
    def get(self, table: str, key: bytes) -> Optional[bytes]:
        """Get a value by key from a table"""
        table_bytes = table.encode('utf-8')
        key_array = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
        out_value = ctypes.POINTER(ctypes.c_uint8)()
        out_len = ctypes.c_size_t()
        
        result = self._lib.byterag_get(
            self._handle, table_bytes,
            key_array, len(key),
            ctypes.byref(out_value), ctypes.byref(out_len)
        )
        
        if result == -4:  # byterag_ERR_NOT_FOUND
            return None
        elif result != 0:
            raise RuntimeError(f"Get failed with error code: {result}")
        
        value = bytes(ctypes.cast(
            out_value, ctypes.POINTER(ctypes.c_uint8 * out_len.value)
        ).contents)
        self._lib.byterag_free_value(out_value, out_len)
        return value
    
    def delete(self, table: str, key: bytes) -> None:
        """Delete a key from a table"""
        table_bytes = table.encode('utf-8')
        key_array = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
        
        result = self._lib.byterag_delete(
            self._handle, table_bytes,
            key_array, len(key)
        )
        
        if result != 0:
            raise RuntimeError(f"Delete failed with error code: {result}")

    # ═══════════════════════════════════════════════════
    # Batch Operations
    # ═══════════════════════════════════════════════════

    def scan(self, table: str) -> List[Tuple[bytes, bytes]]:
        """Scan all key-value pairs in a table"""
        table_bytes = table.encode('utf-8')
        out_result = ctypes.c_void_p()

        result = self._lib.byterag_scan(
            self._handle, table_bytes, ctypes.byref(out_result)
        )
        if result != 0:
            raise RuntimeError(f"Scan failed with error code: {result}")

        return self._read_scan_result(out_result)

    def range(self, table: str, start_key: bytes, end_key: bytes) -> List[Tuple[bytes, bytes]]:
        """Scan a range of keys [start_key, end_key)"""
        table_bytes = table.encode('utf-8')
        start_arr = (ctypes.c_uint8 * len(start_key)).from_buffer_copy(start_key)
        end_arr = (ctypes.c_uint8 * len(end_key)).from_buffer_copy(end_key)
        out_result = ctypes.c_void_p()

        result = self._lib.byterag_range(
            self._handle, table_bytes,
            start_arr, len(start_key),
            end_arr, len(end_key),
            ctypes.byref(out_result)
        )
        if result != 0:
            raise RuntimeError(f"Range scan failed with error code: {result}")

        return self._read_scan_result(out_result)

    def _read_scan_result(self, scan_handle: ctypes.c_void_p) -> List[Tuple[bytes, bytes]]:
        """Read entries from a scan result handle and free it"""
        count = self._lib.byterag_scan_result_count(scan_handle)
        entries = []

        for i in range(count):
            key_ptr = ctypes.POINTER(ctypes.c_uint8)()
            key_len = ctypes.c_size_t()
            val_ptr = ctypes.POINTER(ctypes.c_uint8)()
            val_len = ctypes.c_size_t()

            self._lib.byterag_scan_result_key(
                scan_handle, i, ctypes.byref(key_ptr), ctypes.byref(key_len)
            )
            self._lib.byterag_scan_result_value(
                scan_handle, i, ctypes.byref(val_ptr), ctypes.byref(val_len)
            )

            k = bytes(ctypes.cast(
                key_ptr, ctypes.POINTER(ctypes.c_uint8 * key_len.value)
            ).contents)
            v = bytes(ctypes.cast(
                val_ptr, ctypes.POINTER(ctypes.c_uint8 * val_len.value)
            ).contents)
            entries.append((k, v))

        self._lib.byterag_scan_result_free(scan_handle)
        return entries

    # ═══════════════════════════════════════════════════
    # Utility Operations
    # ═══════════════════════════════════════════════════
    
    def count(self, table: str) -> int:
        """Count rows in a table"""
        table_bytes = table.encode('utf-8')
        out_count = ctypes.c_size_t()
        
        result = self._lib.byterag_count(
            self._handle, table_bytes, ctypes.byref(out_count)
        )
        
        if result != 0:
            raise RuntimeError(f"Count failed with error code: {result}")
        
        return out_count.value
    
    def flush(self) -> None:
        """Flush database to disk"""
        result = self._lib.byterag_flush(self._handle)
        if result != 0:
            raise RuntimeError(f"Flush failed with error code: {result}")

    def table_names(self) -> List[str]:
        """Get all table names"""
        out_list = ctypes.c_void_p()
        result = self._lib.byterag_table_names(self._handle, ctypes.byref(out_list))
        if result != 0:
            raise RuntimeError(f"Table names failed with error code: {result}")

        count = self._lib.byterag_string_list_count(out_list)
        names = []
        for i in range(count):
            str_ptr = ctypes.POINTER(ctypes.c_uint8)()
            str_len = ctypes.c_size_t()
            self._lib.byterag_string_list_get(
                out_list, i, ctypes.byref(str_ptr), ctypes.byref(str_len)
            )
            name = bytes(ctypes.cast(
                str_ptr, ctypes.POINTER(ctypes.c_uint8 * str_len.value)
            ).contents).decode('utf-8')
            names.append(name)

        self._lib.byterag_string_list_free(out_list)
        return names

    def gc(self) -> int:
        """Run garbage collection"""
        out_deleted = ctypes.c_size_t()
        result = self._lib.byterag_gc(self._handle, ctypes.byref(out_deleted))
        if result != 0:
            raise RuntimeError(f"GC failed with error code: {result}")
        return out_deleted.value

    def is_encrypted(self) -> bool:
        """Check if the database is encrypted"""
        return self._lib.byterag_is_encrypted(self._handle) != 0

    # ═══════════════════════════════════════════════════
    # SQL Operations
    # ═══════════════════════════════════════════════════

    def execute_sql(self, sql: str) -> int:
        """Execute a SQL statement (SELECT/INSERT/UPDATE/DELETE)"""
        sql_bytes = sql.encode('utf-8')
        out_affected = ctypes.c_size_t()
        result = self._lib.byterag_execute_sql(
            self._handle, sql_bytes, ctypes.byref(out_affected)
        )
        if result != 0:
            raise RuntimeError(f"SQL execution failed with error code: {result}")
        return out_affected.value

    # ═══════════════════════════════════════════════════
    # Index Operations
    # ═══════════════════════════════════════════════════

    def create_index(self, table: str, column: str) -> None:
        """Create an index on a table column"""
        result = self._lib.byterag_create_index(
            self._handle, table.encode('utf-8'), column.encode('utf-8')
        )
        if result != 0:
            raise RuntimeError(f"Create index failed with error code: {result}")

    def drop_index(self, table: str, column: str) -> None:
        """Drop an index from a table column"""
        result = self._lib.byterag_drop_index(
            self._handle, table.encode('utf-8'), column.encode('utf-8')
        )
        if result != 0:
            raise RuntimeError(f"Drop index failed with error code: {result}")

    def has_index(self, table: str, column: str) -> bool:
        """Check if an index exists on a table column"""
        return self._lib.byterag_has_index(
            self._handle, table.encode('utf-8'), column.encode('utf-8')
        ) != 0

    # ═══════════════════════════════════════════════════
    # Snapshot Operations
    # ═══════════════════════════════════════════════════

    def save_to_file(self, path: str) -> None:
        """Save the database to a file"""
        result = self._lib.byterag_save_to_file(
            self._handle, path.encode('utf-8')
        )
        if result != 0:
            raise RuntimeError(f"Save failed with error code: {result}")

    # ═══════════════════════════════════════════════════
    # MVCC Operations
    # ═══════════════════════════════════════════════════

    def current_timestamp(self) -> int:
        """Get the current MVCC timestamp"""
        return self._lib.byterag_current_timestamp(self._handle)

    def allocate_commit_ts(self) -> int:
        """Allocate a new commit timestamp"""
        return self._lib.byterag_allocate_commit_ts(self._handle)

    def insert_versioned(self, table: str, key: bytes, value: bytes, commit_ts: int) -> None:
        """Insert a versioned key-value pair (MVCC)"""
        table_bytes = table.encode('utf-8')
        key_array = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
        value_array = (ctypes.c_uint8 * len(value)).from_buffer_copy(value)

        result = self._lib.byterag_insert_versioned(
            self._handle, table_bytes,
            key_array, len(key),
            value_array, len(value),
            commit_ts
        )
        if result != 0:
            raise RuntimeError(f"Versioned insert failed with error code: {result}")

    def get_snapshot(self, table: str, key: bytes, read_ts: int) -> Optional[bytes]:
        """Read a specific version of a key (Snapshot Read)"""
        table_bytes = table.encode('utf-8')
        key_array = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
        out_value = ctypes.POINTER(ctypes.c_uint8)()
        out_len = ctypes.c_size_t()

        result = self._lib.byterag_get_snapshot(
            self._handle, table_bytes,
            key_array, len(key),
            read_ts,
            ctypes.byref(out_value), ctypes.byref(out_len)
        )

        if result == -4:  # byterag_ERR_NOT_FOUND
            return None
        elif result != 0:
            raise RuntimeError(f"Snapshot read failed with error code: {result}")

        value = bytes(ctypes.cast(
            out_value, ctypes.POINTER(ctypes.c_uint8 * out_len.value)
        ).contents)
        self._lib.byterag_free_value(out_value, out_len)
        return value

    # ═══════════════════════════════════════════════════
    # Transaction & Lifecycle
    # ═══════════════════════════════════════════════════
    
    def close(self) -> None:
        """Close the database and free resources"""
        if self._handle:
            self._lib.byterag_close(self._handle)
            self._handle = None
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
    
    def __del__(self):
        self.close()
    
    # ═══════════════════════════════════════════════════
    # Internal Library Loading
    # ═══════════════════════════════════════════════════

    def _load_library(self) -> ctypes.CDLL:
        """Load the DBX shared library"""
        system = platform.system()
        if system == "Windows":
            lib_name = "byterag_ffi.dll"
        elif system == "Darwin":
            lib_name = "libbyterag_ffi.dylib"
        else:
            lib_name = "libbyterag_ffi.so"
        
        search_paths = [
            Path(__file__).parent.parent.parent.parent / "core" / "byterag-ffi" / "target" / "release" / lib_name,
            Path(__file__).parent / lib_name,
            lib_name,
        ]
        
        for path in search_paths:
            try:
                return ctypes.CDLL(str(path))
            except OSError:
                continue
        
        raise RuntimeError(f"Could not find DBX library ({lib_name})")
    
    def _setup_functions(self) -> None:
        """Setup function signatures"""
        # Constructors
        self._lib.byterag_open.argtypes = [ctypes.c_char_p]
        self._lib.byterag_open.restype = ctypes.c_void_p

        self._lib.byterag_open_in_memory.argtypes = []
        self._lib.byterag_open_in_memory.restype = ctypes.c_void_p

        self._lib.byterag_load_from_file.argtypes = [ctypes.c_char_p]
        self._lib.byterag_load_from_file.restype = ctypes.c_void_p
        
        # CRUD
        self._lib.byterag_insert.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
        ]
        self._lib.byterag_insert.restype = ctypes.c_int
        
        self._lib.byterag_get.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.POINTER(ctypes.c_uint8)),
            ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_get.restype = ctypes.c_int
        
        self._lib.byterag_delete.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
        ]
        self._lib.byterag_delete.restype = ctypes.c_int

        # Scan/Range
        self._lib.byterag_scan.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p),
        ]
        self._lib.byterag_scan.restype = ctypes.c_int

        self._lib.byterag_range.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_void_p),
        ]
        self._lib.byterag_range.restype = ctypes.c_int

        self._lib.byterag_scan_result_count.argtypes = [ctypes.c_void_p]
        self._lib.byterag_scan_result_count.restype = ctypes.c_size_t

        self._lib.byterag_scan_result_key.argtypes = [
            ctypes.c_void_p, ctypes.c_size_t,
            ctypes.POINTER(ctypes.POINTER(ctypes.c_uint8)),
            ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_scan_result_key.restype = ctypes.c_int

        self._lib.byterag_scan_result_value.argtypes = [
            ctypes.c_void_p, ctypes.c_size_t,
            ctypes.POINTER(ctypes.POINTER(ctypes.c_uint8)),
            ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_scan_result_value.restype = ctypes.c_int

        self._lib.byterag_scan_result_free.argtypes = [ctypes.c_void_p]
        self._lib.byterag_scan_result_free.restype = None
        
        # Utility
        self._lib.byterag_count.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_count.restype = ctypes.c_int
        
        self._lib.byterag_flush.argtypes = [ctypes.c_void_p]
        self._lib.byterag_flush.restype = ctypes.c_int

        self._lib.byterag_table_names.argtypes = [
            ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p),
        ]
        self._lib.byterag_table_names.restype = ctypes.c_int

        self._lib.byterag_string_list_count.argtypes = [ctypes.c_void_p]
        self._lib.byterag_string_list_count.restype = ctypes.c_size_t

        self._lib.byterag_string_list_get.argtypes = [
            ctypes.c_void_p, ctypes.c_size_t,
            ctypes.POINTER(ctypes.POINTER(ctypes.c_uint8)),
            ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_string_list_get.restype = ctypes.c_int

        self._lib.byterag_string_list_free.argtypes = [ctypes.c_void_p]
        self._lib.byterag_string_list_free.restype = None

        self._lib.byterag_gc.argtypes = [
            ctypes.c_void_p, ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_gc.restype = ctypes.c_int

        self._lib.byterag_is_encrypted.argtypes = [ctypes.c_void_p]
        self._lib.byterag_is_encrypted.restype = ctypes.c_int

        # SQL
        self._lib.byterag_execute_sql.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_execute_sql.restype = ctypes.c_int

        # Index
        self._lib.byterag_create_index.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ]
        self._lib.byterag_create_index.restype = ctypes.c_int

        self._lib.byterag_drop_index.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ]
        self._lib.byterag_drop_index.restype = ctypes.c_int

        self._lib.byterag_has_index.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p,
        ]
        self._lib.byterag_has_index.restype = ctypes.c_int

        # Snapshot
        self._lib.byterag_save_to_file.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
        self._lib.byterag_save_to_file.restype = ctypes.c_int

        # MVCC
        self._lib.byterag_current_timestamp.argtypes = [ctypes.c_void_p]
        self._lib.byterag_current_timestamp.restype = ctypes.c_uint64

        self._lib.byterag_allocate_commit_ts.argtypes = [ctypes.c_void_p]
        self._lib.byterag_allocate_commit_ts.restype = ctypes.c_uint64

        self._lib.byterag_insert_versioned.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.c_uint64,
        ]
        self._lib.byterag_insert_versioned.restype = ctypes.c_int

        self._lib.byterag_get_snapshot.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.c_uint64,
            ctypes.POINTER(ctypes.POINTER(ctypes.c_uint8)),
            ctypes.POINTER(ctypes.c_size_t),
        ]
        self._lib.byterag_get_snapshot.restype = ctypes.c_int
        
        # Memory
        self._lib.byterag_free_value.argtypes = [
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
        ]
        self._lib.byterag_free_value.restype = None
        
        self._lib.byterag_close.argtypes = [ctypes.c_void_p]
        self._lib.byterag_close.restype = None
        
        # Transaction
        self._lib.byterag_begin_transaction.argtypes = [ctypes.c_void_p]
        self._lib.byterag_begin_transaction.restype = ctypes.c_void_p
        
        self._lib.byterag_transaction_insert.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
        ]
        self._lib.byterag_transaction_insert.restype = ctypes.c_int
        
        self._lib.byterag_transaction_delete.argtypes = [
            ctypes.c_void_p, ctypes.c_char_p,
            ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t,
        ]
        self._lib.byterag_transaction_delete.restype = ctypes.c_int
        
        self._lib.byterag_transaction_commit.argtypes = [ctypes.c_void_p]
        self._lib.byterag_transaction_commit.restype = ctypes.c_int
        
        self._lib.byterag_transaction_rollback.argtypes = [ctypes.c_void_p]
        self._lib.byterag_transaction_rollback.restype = None

