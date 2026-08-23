from importlib.metadata import version

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
__version__ = version("pakdo")
