# Testing Guide — `pakdo-core`

This document explains the testing strategy, conventions, and rules for `pakdo-core`.
It is intended to help contributors (human or AI) write consistent, non-redundant tests.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Testing Philosophy](#testing-philosophy)
3. [Test Layout](#test-layout)
4. [Where to Put a New Test](#where-to-put-a-new-test)
5. [Test File Reference](#test-file-reference)
6. [Fixtures](#fixtures)
7. [Dev Dependencies](#dev-dependencies)
8. [Code Examples](#code-examples)
9. [Running the Tests](#running-the-tests)

---

## Architecture Overview

`pakdo-core` is a file conversion library. Its internal flow is:

```
convert(input, output, target_ext, library)
  │
  ├─ get_conversion_string(input, output)   → e.g. "jpg>png"
  │    └─ get_extension_from_file(input)    → detects type via magic bytes (infer)
  │
  ├─ ROUTES["jpg>png"]                      → resolves to a ConvertFn
  │   or LIBRARY["image"]                   → resolves to a ConvertFn (--lib flag)
  │
  └─ ConvertFn(input, output)               → e.g. Image::process()
```

**Key components:**

| Component | File | Visibility |
|---|---|---|
| `convert()` | `src/lib.rs` | `pub` — main entry point |
| `get_conversion_string()` | `src/lib.rs` | private |
| `get_extension_from_file()` | `src/utils.rs` | `pub` |
| `ROUTES`, `LIBRARY` | `src/config.rs` | private static maps |
| `Converter` trait | `src/modules/converter.rs` | `pub` |
| `Image::process()` | `src/modules/image/image.rs` | `pub` (via trait) |

---

## Testing Philosophy

The test suite is organized around two distinct concerns:

### 1. Integration tests (via `convert()`)
**"What can the application do from the user's perspective?"**

These tests call `convert()` — the public entry point — and verify the complete chain:
routing → converter → output file. They are the most valuable tests because they reflect
real usage and catch regressions anywhere in the pipeline.

### 2. Converter-level tests (via `<ConcreteConverter>::process()`)
**"What is the contract of this specific converter?"**

These tests call a converter directly, bypassing the routing layer. They are useful
for documenting which formats a given converter supports, especially formats that are
only accessible via the `--lib` flag and therefore not exercised by `convert_tests.rs`.

> **Rule:** Do NOT write a converter-level test that duplicates a `convert_tests.rs` test.
> If `convert_tests.rs` already calls `convert(input.jpg, output.png, ...)` and it passes
> through `Image::process`, adding `test_process_jpg_to_png` in `image_tests.rs` is redundant.

### Decision tree

```
Is the function private?
  YES → inline #[cfg(test)] in the same source file
  NO  → external file in tests/

Is it testing the full user-facing behavior (routing included)?
  YES → tests/convert_tests.rs

Is it testing a specific converter's capabilities, isolated from routing?
  YES → tests/<converter>_tests.rs
  BUT only if: the format is NOT already covered by convert_tests.rs
           OR: the test exercises error handling specific to the converter itself
```

---

## Test Layout

```
core/
├── src/
│   ├── lib.rs              ← inline #[cfg(test)] for private get_conversion_string()
│   ├── utils.rs            ← no inline tests (function is public → external file)
│   └── modules/
│       └── image/
│           └── image.rs    ← no inline tests (method is public → external file)
└── tests/
    ├── common/
    │   └── mod.rs          ← shared test helpers (fixture_path, downcast_pakdo_error)
    ├── fixtures/           ← real image files used by tests
    │   ├── image.jpg
    │   └── image.png
    ├── convert_tests.rs    ← integration tests via convert()
    ├── image_tests.rs      ← Image converter contract (isolated from routing)
    └── utils_tests.rs      ← get_extension_from_file() integration tests
```

---

## Where to Put a New Test

### Adding a test for a **private** function

Write it inline in the source file using a `#[cfg(test)]` module:

```rust
// src/some_module.rs

fn my_private_fn(x: u32) -> u32 { x * 2 }

#[cfg(test)]
mod tests {
    use super::*;  // gives access to private items in the parent module

    #[test]
    fn test_my_private_fn() {
        assert_eq!(my_private_fn(3), 6);
    }
}
```

### Adding a test for a **new converter** (e.g., `VideoFFmpeg`)

1. Create `tests/video_tests.rs`
2. Only add tests for formats/behaviors NOT already covered by `convert_tests.rs`
3. Add a comment at the top explaining which formats are intentionally not tested here
   (because they are covered end-to-end in `convert_tests.rs`)

```rust
// tests/video_tests.rs

// mp4→webm is covered end-to-end in convert_tests.rs.
// Tests here cover formats only reachable via --lib video,
// or error conditions specific to the VideoFFmpeg converter.

use pakdo_core::modules::video::VideoFFmpeg;
use pakdo_core::modules::converter::Converter;
```

### Adding a test for a **new route** in `ROUTES`

Add it to `tests/convert_tests.rs`. Always test:
- The happy path (input exists, output is created)
- At least one error case specific to that route if applicable

### Adding a test for a **new public utility function**

Add it to the relevant `tests/*_tests.rs` file, or create a new one if none fits.

---

## Test File Reference

### `src/lib.rs` — Inline unit tests

**Tests:** `get_conversion_string()` (private)

| Test name | What it checks |
|---|---|
| `test_conversion_string_jpg_to_png` | Returns `"jpg>png"` for a real JPG input |
| `test_conversion_string_png_to_jpg` | Returns `"png>jpg"` for a real PNG input |
| `test_conversion_string_output_without_extension_returns_error` | Returns `UnknownFileExtension` when output has no extension |

---

### `tests/common/mod.rs` — Shared test helpers

| Helper | Description |
|---|---|
| `fixture_path(name)` | Resolves a fixture file path relative to `CARGO_MANIFEST_DIR` |
| `downcast_pakdo_error(result)` | Unwraps an error `Result` and downcasts it to `PakdoError` |

Import in test files with:
```rust
mod common;
use common::{fixture_path, downcast_pakdo_error};
```

---

### `tests/convert_tests.rs` — Integration tests

**Tests:** `convert()` public API — routing + conversion end-to-end

| Test name | What it checks |
|---|---|
| `test_convert_no_output_no_format_returns_error` | Both `output` and `target_extension` are `None` → `PakdoError::InvalidArguments` |
| `test_convert_unknown_library_returns_error` | `--lib nonexistent` → `PakdoError::UnknownLibrary` |
| `test_convert_unsupported_route_returns_error` | Route not in `ROUTES` (e.g. `png>gif`) → `PakdoError::ExtensionNotSupported` |
| `test_convert_output_file_with_extension_and_target_ext_returns_error` | Output has extension + `target_extension` provided → `PakdoError::InvalidArguments` |
| `test_convert_jpg_to_png_with_output_path` | Happy path: JPG → PNG, output file is created |
| `test_convert_png_to_jpg_with_output_dir_and_extension` | Output path is a dir + `target_extension` → filename built from input stem |

---

### `tests/image_tests.rs` — Image converter contract

**Tests:** `Image::process()` in isolation from routing

> Only add tests here for behaviors that `convert_tests.rs` cannot observe.
> Happy-path format conversions covered by `convert_tests.rs` must NOT be duplicated here.

| Test name | What it checks |
|---|---|
| `test_process_nonexistent_file_returns_error` | Converter returns an error when the input file does not exist |

---

### `tests/utils_tests.rs` — Extension detection

**Tests:** `get_extension_from_file()` public utility

| Test name | What it checks |
|---|---|
| `test_get_extension_known_png` | PNG magic bytes → returns `"png"` |
| `test_get_extension_known_jpg` | JPEG magic bytes → returns `"jpg"` |
| `test_get_extension_file_not_found` | Non-existent path → `PakdoError::FileNotFound` |
| `test_get_extension_unknown_type` | Empty file → `PakdoError::UnknownFileExtension` |

---

## Fixtures

Real image files are stored in `core/tests/fixtures/`. They are used by all test files
that need valid input images (e.g. for conversion tests).

```
tests/fixtures/
├── image.jpg    ← valid JPEG (1×1 pixel minimum)
└── image.png    ← valid PNG  (1×1 pixel minimum)
```

**Important:** Relying on the process working directory can be fragile across workspaces, IDE test runners, or custom test harnesses. Always resolve fixture paths relative to `env!("CARGO_MANIFEST_DIR")`. The shared helper `fixture_path()` in `tests/common/mod.rs` does this:

```rust
mod common;
use common::fixture_path;

#[test]
fn test_something() {
    let input = fixture_path("image.jpg");
    // ...
}
```

**Adding a new fixture:** place the file in `tests/fixtures/` and use a descriptive name
if the file is format-specific (e.g. `sample_cmyk.jpg` for a CMYK edge case).

---

## Dev Dependencies

```toml
# core/Cargo.toml
[dev-dependencies]
tempfile = "3"
```

### Why `tempfile`?

Tests that write output files use `tempfile::TempDir` to create an isolated temporary
directory per test. The directory and all its contents are automatically deleted when
the `TempDir` value is dropped (end of test, including panics).

```rust
use tempfile::TempDir;

#[test]
fn test_something() {
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.png");  // unique path per test run
    // ... run conversion ...
    assert!(output.exists());
    // dir is dropped here → output is deleted automatically
}
```

**Do not** hardcode paths like `/tmp/out.png` or `/tmp/missing.xyz`. For testing non-existent files, construct a path inside a `TempDir` that is guaranteed not to exist:

```rust
let dir = TempDir::new().unwrap();
let missing = dir.path().join("missing.xyz");
assert!(!missing.exists());
```

For tests that need a temporary **file** with specific content (e.g. magic bytes),
use `tempfile::NamedTempFile`:

```rust
use tempfile::NamedTempFile;
use std::io::Write;

let mut f = NamedTempFile::new().unwrap();
f.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]).unwrap(); // PNG magic bytes
let result = get_extension_from_file(f.path());
```

---

## Code Examples

### Minimal integration test (convert)

```rust
mod common;

use common::fixture_path;
use pakdo_core::convert;
use tempfile::TempDir;

#[test]
fn test_convert_format_a_to_format_b() {
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.b");
    let result = convert(
        &fixture_path("input.a"),
        Some(&output),
        None,
        None,
    );
    result.expect("conversion should succeed");
    assert!(output.exists());
}
```

### Minimal error test (typed matching)

```rust
mod common;

use common::downcast_pakdo_error;
use pakdo_core::convert;
use pakdo_core::errors::PakdoError;
use std::path::Path;

#[test]
fn test_convert_returns_specific_error() {
    let err = downcast_pakdo_error(convert(Path::new("any.jpg"), None, None, None));
    assert!(matches!(err, PakdoError::InvalidArguments(_)));
}
```

### Minimal converter isolation test

```rust
mod common;

use common::fixture_path;
use pakdo_core::modules::my_converter::MyConverter;
use pakdo_core::modules::converter::Converter;
use tempfile::TempDir;

#[test]
fn test_my_converter_specific_behavior() {
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.ext");
    let result = MyConverter::process(
        &fixture_path("input.ext"),
        &output,
    );
    result.expect("converter process should succeed");
    assert!(output.exists());
}
```

---

## Running the Tests

```bash
# Run all tests for pakdo-core
cargo test -p pakdo-core

# Run a specific test file
cargo test -p pakdo-core --test convert_tests
cargo test -p pakdo-core --test image_tests
cargo test -p pakdo-core --test utils_tests

# Run a specific test by name
cargo test -p pakdo-core test_convert_jpg_to_png

# Run with output (useful for debugging)
cargo test -p pakdo-core -- --nocapture
```
