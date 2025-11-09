# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**turbopfor_rs** is a high-performance Rust wrapper for the [TurboPFor Integer Compression](https://github.com/powturbo/TurboPFor-Integer-Compression) C library. It provides bindings to what the authors claim is the "fastest integer compression" library.

- **License**: GPL-2.0
- **Version**: 0.4.2
- **Architecture**: Rust wrapper around a C library with auto-generated FFI bindings
- **Focus**: Maximum performance, not safety

## Common Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
make lib                 # Build the library (also regenerates bindings)
```

### Testing
```bash
cargo test --release     # Run all tests (run in release mode for accuracy)
./test.sh                # Continuous testing loop
```

### Code Generation
The Rust bindings are auto-generated from C headers via Python scripts:
```bash
python3 make.py ic       # Generate raw FFI bindings (src/ic.rs)
python3 make.py lib      # Generate wrapper functions (src/lib.rs)
make clean               # Remove generated files
```

## Architecture

### Core Components (src/)

1. **lib.rs** (auto-generated, ~2,200 lines)
   - Main public API
   - Safe Rust wrappers around C functions
   - Contains the `p4` module with direct encoding/decoding functions

2. **codec.rs** (~350 lines)
   - `Width` trait: Buffer size calculations (W, W128v, W256v)
   - `Codec` trait: Type-safe encode/decode operations
   - Provides abstraction layer over raw functions

3. **generic.rs** (~300 lines, v0.4.1+)
   - Trait-based generic API via `Encoding` trait
   - Four implementations: `StandardEncoding`, `IncreasingEncoding`, `StrictlyIncreasingEncoding`, `ZigZagEncoding`
   - Better for generic programming than direct codec functions

4. **ic.rs** (auto-generated, ~350 lines)
   - Unsafe FFI bindings to C library
   - Direct mappings to C functions
   - Not meant for direct use

5. **test.rs** (~350 lines)
   - Comprehensive test suite
   - Property-based testing with random data (up to 1MB, 128 iterations)
   - Tests all width variants and data types

### Build System

- **Custom build script** (`build.rs`): Runs during Cargo build, executes `make`, links static library
- **C library**: Compiled from `TurboPFor-Integer-Compression` submodule to `vendor/turbopfor/libic.a`
- **Python generators** (`make.py`, `make2.py`): Parse C headers and generate Rust bindings
- **Sanitized headers** (`c_headers/`): Manually cleaned C headers for parser compatibility

## Critical Information

### ⚠️ Buffer Size Requirements (MANDATORY)

TurboPFor does **NOT perform bounds checking** and will write beyond array bounds if buffer is too small, resulting in **segfaults**.

**Encoding:**
- Use `W::enc_buf_size::<T>(n)` to calculate minimum buffer size
- TurboPFor will not read beyond input bounds

**Decoding:**
- Allocate 32 extra integers OR use `W::dec_buf_len::<T>(n)`
- TurboPFor may write up to 32 integers beyond output buffer

**Example:**
```rust
use turbopfor_rs::codec::{W, Codec, Width};

let input = vec![0u32, 1, 2, 3];
let mut output = vec![0u32; input.len() + 32];  // +32 for safety
let mut buf = vec![0u8; 1024];

let size_enc = Codec::<W>::enc(&input, &mut buf);
let size_dec = Codec::<W>::dec(&mut buf, input.len(), &mut output);
```

### API Usage

**Width types** (from `turbopfor_rs::codec`):
- `W`: Standard width
- `W128v`: 128-bit SIMD
- `W256v`: 256-bit SIMD

**Data types supported**: u8, u16, u32, u64

**Encoding schemes**:
- Standard encoding for unsorted lists
- Delta encoding for increasing lists
- Delta1 encoding for strictly increasing lists
- ZigZag encoding for signed integers

## Development Notes

### Testing
- All tests are in `test.rs`
- Comprehensive property-based tests with random data
- Tests all combinations of widths, types, and encoding schemes
- CI runs on GitHub Actions (Ubuntu, stable Rust)

### No Linting Configured
- No `rustfmt.toml` or `.rustfmt.toml`
- No `.clippy.toml` or Clippy in CI
- Only basic Rust compiler checks in CI pipeline

### Function Naming Convention
C library uses compact naming: `{vb|p4|bit|vs}[n][d|d1|f|fm|z]{enc/dec|pack/unpack}[|128V|256V][8|16|32|64]`

| Code | Meaning |
| ---- | ------- |
| vb   | Variable byte |
| p4   | TurboPFor |
| vs   | Variable simple |
| bit  | Bit packing |
| n    | High-level array functions |

| Code | Meaning |
| ---- | ------- |
| ''   | Unsorted lists |
| 'd'  | Delta encoding (increasing) |
| 'd1' | Delta encoding (strictly increasing) |
| 'f'  | FOR encoding (sorted) |
| 'z'  | ZigZag encoding (signed) |

### Coverage Status
Only `vp4` functions are currently wrapped. Other codecs (bitpack, eliasfano, fp, vint, vsimple) have bindings but no Rust wrappers.

## External Resources

- [Original TurboPFor library](https://github.com/powturbo/TurboPFor-Integer-Compression)
- [Michael Stapelberg's analysis (2019)](https://michael.stapelberg.ch/posts/2019-02-05-turbopfor-analysis/)
- [icapp utility](https://github.com/powturbo/TurboPFor-Integer-Compression/blob/master/icapp.c): Benchmarking tool in the C library
