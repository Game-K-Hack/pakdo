"""Pakdo - optimized, 100% local file converter."""

from pakdo._native import (
    convert,
    ExtensionNotSupportedError,
    FileNotFoundError,
    FileOpenError,
    InvalidArgumentsError,
    PakdoBaseError,
    UnknownFileExtensionError,
    UnknownLibraryError,
)

__all__ = [
    "convert",
    "PakdoBaseError",
    "InvalidArgumentsError",
    "FileNotFoundError",
    "FileOpenError",
    "UnknownFileExtensionError",
    "UnknownLibraryError",
    "ExtensionNotSupportedError",
]
__version__ = "0.1.0"
