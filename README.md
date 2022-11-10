![GitHub](https://img.shields.io/github/license/mayeranalytics/turbopfor_rs)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/mayeranalytics/turbopfor_rs?include_prereleases)
![CI](https://github.com/mayeranalytics/turbopfor_rs/actions/workflows/ci.yml/badge.svg)

# turbopfor\_rs

This is a wrapper for [TurboPFor Integer Compression](https://github.com/powturbo/TurboPFor-Integer-Compression).
The authors claim it to be the "Fastest Integer Compression", and support their claim with their own suite of benchmarks.
(In general, the use of superlatives in TorboPFor's readme is rather irritating.)

[Michael Stapelberg](https://github.com/stapelberg/) performed an independent analysis: [TurboPFor: An analysis (2019)](https://michael.stapelberg.ch/posts/2019-02-05-turbopfor-analysis/).

Use at your own risk!

## Caveats
- The [turbopfor](https://github.com/powturbo/TurboPFor-Integer-Compression) library appears to be abandoned, the authors do not respond anymore. The documentation is lacking, and the source code is buggy.
- The tests pass consistently **if** the Zippenfenig patch is applied
- Write buffer of sufficient size must be allocated, otherwise turbopfor_rs may read/write beyond allocated memory resulting in segfaults of other problems

## Installation
Clone the [TurboPFor](https://github.com/powturbo/TurboPFor-Integer-Compression) repo.

Then apply the patch
```shell
patch -u TurboPFor-Integer-Compression/vp4c.c -i TurboPFor-Integer-Compression.patch
```

After compilation create a directory `deps/` and place the compiled turbopfor library `libic.a` into it. Then use `cargo build` and `cargo test` as usual. (Use `--release` for testing if you want speedy execution.)

Todo: Automate all this with a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

### Acknowledgements

The bug in `vp4c.c` was found by [Patrick Zippenfenig](https://github.com/patrick-zippenfenig).

### All steps
```shell
mkdir deps/
git clone https://github.com/powturbo/TurboPFor-Integer-Compression.git 
patch -u TurboPFor-Integer-Compression/vp4c.c -i TurboPFor-Integer-Compression.patch
cd TurboPFor-Integer-Compression
make -j
cp libic.a ../deps
cd ..
cargo test --release
``` 

## Function name convention

```ascii
{vb | p4 | bit | vs}[n][d | d1 | f | fm | z ]{enc/dec | pack/unpack}[| 128V | 256V][8 | 16 | 32 | 64]:
```

| code | meaning                                      |
| ---- | -------------------------------------------- |
| vb   | variable byte                                |
| p4   | turbopfor                                    |
| vs   | variable simple                              |
| bit  | bit packing                                  |
| n    | high level array functions for large arrays. |

| code | meaning                                                              |
| ---- | -------------------------------------------------------------------- |
| ''   | encoding for unsorted integer lists                                  |
| 'd'  | delta encoding for increasing integer lists (sorted w/ duplicate)    |
| 'd1' | delta encoding for strictly increasing integer lists (sorted unique) |
| 'f'  | FOR encoding for sorted integer lists                                |
| 'z'  | ZigZag encoding for unsorted integer lists                           |

| code              | meaning                   |
| ----------------- | ------------------------- |
| 'enc' or 'pack'   | encode or bitpack         |
| 'dec' or 'unpack' | decode or bitunpack       |
| 'NN'              | integer size (8/16/32/64) |

# Available Functions

| enc/dec | 8   | 16  | 32  | 64  |
| ------- |:---:|:---:|:---:|:---:|
| -       | x   | x   | x   | x   |
| 128v    |     | x   | x   |     |
| 256v    |     |     | x   |     |

Exceptions:

- enc/dec128v64

- in addition to enc/dec256v there is a 'w' version enc/dec256w

# Alignment, input and buffer sizes

- [Alignment and padding, Issue #59]([Alignment and tailing padding requirements for the decoder APIs · Issue #59 · powturbo/TurboPFor-Integer-Compression · GitHub](https://github.com/powturbo/TurboPFor-Integer-Compression/issues/59):
  
  - Alignment for input and output is not required
  
  - Trailing bytes must no be set to anything. Just call encode and store your the output buffer with the  length returned.
  
  - p4ndec256v32 and p4nzdec256v32 can read and write up to 32 integers beyond the input/output buffer.  For encoding you can use:  
    #define P4NENC256_BOUND(n) ((n + 255) /256 + (n + 32) * sizeof(uint32_t))  
    For decoding just add 32*4 bytes when allocating the input/output buffer

- [Out-of-bounds memory access causes segmentation fault](https://github.com/powturbo/TurboPFor-Integer-Compression/issues/80)

- [Recommended output buffer estimation, Issue # 25]([recommended output buffer estimation · Issue #25 · powturbo/TurboPFor-Integer-Compression · GitHub](https://github.com/powturbo/TurboPFor-Integer-Compression/issues/25)):
  
  - For the bitpacking functions of type T you need 1 + (MaxBits*(N+32)+7)/8 bytes per block
  
  - Same for the TurboPFor (p4enc) functions, as p4enc never compress worse than bitpacking
  
  - In general it is simpler to use : 1 + (sizeof(T)*(N+32) per block (ex. 128/256).
  
  - 32 elements as overhead to avoid segfault.

Example from [Issue #25]([recommended output buffer estimation · Issue #25 · powturbo/TurboPFor-Integer-Compression · GitHub](https://github.com/powturbo/TurboPFor-Integer-Compression/issues/25)):

```c
static size_t encode(uint32_t *input, size_t n, FILE *f) {
    // 1 byte overhead per 128 integers
    unsigned char *out = malloc((n+127)/128+(n+32)*sizeof(uint32_t));
    size_t sz = p4nenc32(input,n,out);
    sz = fwrite(out, 1, sz, f);
    free(out);
    return sz;
}

int main() {
    uint32_t input[32] = {3, 4, 5, 6, 9, 10, 15, 17, 18, 333};
    size_t sz = encode(input, 10);
    fprintf(stderr, "encoded into %lu bytes\n", sz);
}
```
