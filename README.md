![GitHub](https://img.shields.io/github/license/mayeranalytics/turbopfor_rs)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/mayeranalytics/turbopfor_rs?include_prereleases)
![CI](https://github.com/mayeranalytics/turbopfor_rs/actions/workflows/ci.yml/badge.svg)

# turbopfor\_rs

This is a wrapper for [TurboPFor Integer Compression](https://github.com/powturbo/TurboPFor-Integer-Compression).
The authors claim it to be the "Fastest Integer Compression", and support their claim with their own suite of benchmarks.

[Michael Stapelberg](https://github.com/stapelberg/) performed an independent analysis: [TurboPFor: An analysis (2019)](https://michael.stapelberg.ch/posts/2019-02-05-turbopfor-analysis/).

Please consider carefully the following

#### Caveats

- **Critical buffer sizes:** Write buffer of sufficient size must be allocated, otherwise turbopfor_rs may write beyond allocated memory resulting in segfaults. Likewise, when decoding, the input slice must be large enough to support decoding of the required number of integers, otherwise you get segfaults again.
- **License:** GPL v2.

This crate will be published on [crates.io](https://crates.io) when more tests are available. Feedback regarding successes or failures with this library is very welcome!

Tests were performed on Intel Intel Core i7 running Linux and MacOS.

## Acknowledgements

A big thansk to [Patrick Zippenfenig](https://github.com/patrick-zippenfenig) for sharing the the bugfix in [vp4c.c](https://github.com/powturbo/TurboPFor-Integer-Compression/blob/90867ca1169b7af93d908ebefdb28f24cdce79da/vp4c.c), which is  [incorporated](https://github.com/powturbo/TurboPFor-Integer-Compression/commit/21040077ffe3877096c58f41cf62ba715571c7c0) in turbopfor, now.

## Installation

Cargo should automaticall download, patch and build the turbopfor library.

```shell
cargo build
cargo test --release
```

## Usage

First of all, add this line to the `[dependencies]` of your `Cargo.toml`:

```toml
turbopfor_rs = { git="https://github.com/mayeranalytics/turbopfor_rs", 
                 version="0.4" }
```

The functions `enc`, `dec`, `denc`, `ddec`, etc. have different variants depending on the register width that is used.

The phantom types associated with these widths are found in `turbopfor_rs::codec`:

- `W` 
- `W128v`
- `W256`

Each width implements the [turbopfor_rs::codec::Width](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/codec.rs#L4) trait.
`Width` has a function [buf_size](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/codec.rs#L6) that returns the necessary output buffer size (in bytes) for a given input length. If the output buffer is too small you will get segfaults!

For each width the [Codec](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/codec.rs#L33) trait provides the [enc](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/codec.rs#L40), [dec](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/codec.rs#L49), etc., encoder/decoder pairs.

Example:

```rust
use turbopfor_rs::codec::{W, Codec, Width};

fn main() {
    let input = vec![0,1,2,3,4,5,9,7,8,999999u32];
    println!("input: {:?}", &input);

    /// allocate output buffer
    let buflen: usize = W::enc_buf_size::<u32>(input.len());
    let mut output = vec![0u8; buflen];

    // encode
    let l = Codec::<W>::enc(&input, &mut output);

    println!("encoded: {:?}", &output[..l]);
}
```

You can also use the basic wrappers in [turbopfor_rs::p4](https://github.com/mayeranalytics/turbopfor_rs/blob/fbb279c20a883732b6b757a00f863a8537d4a098/src/lib.rs#L5) directly.

Decoding works the same way, but note that you have to provide the number `n` of output integers you wish to decode:

```rust
fn dec(input: &[u8], n: usize, output: &mut [Self]) -> usize;
```

#### Buffer sizes!

Turbopfor does not perform any bounds checks and will read/write as far as it has to. This aspect is not documented and the statements in the Issues are ambiguous.

- Encoding:
  
  - Allocate enough write buffer using `W::enc_buf_size<T>::(n)`
  
  - To the best of our knowledge Turbopfor will not read beyond the bounds of the input array

- Decoding
  
  - Allocate 32 extra integers, or use  `W::dec_buf_len::<T>(n)`
  
  - To the best of our knowledge Turbopfor will not read beyond the bounds of the input

You must ensure that the outputs is are long enough, otherwise you will get segfaults!

(Note: We are adhering to the naming convention that "size" refers to a number of *bytes*, whereas "len" refers to a number of *items* in an array of any type T.)

To illustrate the point, encode the array `[0,1,2,3]` and then decode it:

```rust
let input = vec![0u32, 1, 2, 3];                // encode this
let mut output = vec![0u32; input.len()+32];    // decode into this
let mut buf = vec![0u8; 1024];                  // generous buffer
// encode
let size_enc = Codec::<W>::enc(&input, &mut buf);
// decode
let size_dec = Codec::<W>::dec(&mut buf, input.len(), &mut output);
assert_eq!(size_enc, size_dec);
println!("{:?}", output);
```

The `output` `Vec<u32>` is

```
[0, 1, 2, 3,
 0, 0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0]
```

Clearly, Trubopfor writes more than 4 `u32`s.

### Coverage

|           | bindings | wrapper | tests |     |
| --------- |:--------:| ------- | ----- | --- |
| bitpack   | y        |         |       |     |
| eliasfano | y        |         |       |     |
| fp        | y        |         |       |     |
| vint      | y        |         |       |     |
| vp4       | y        |         |       |     |

So far only the `vp4` functions are wrapped. The `fp` floating point codecs seem interesting, but the experiments with `icapp` (see below) show no benefit whatsoever. We are probably using the fp functions incorrectly.

# Turbpfor_rs internals

## c_headers/

The `TurboPFor` header files are placed in the `c_headers/` directory and manually sanitized (i.e. made fit for the *very* basic C parser used in `make.py`):

- remove everything except for function decls and comments

- replace `/* */` comments with `//` 

- replace the `unsigned` type by `unsigned int`

- remove pragmas such as `__restrict`

- remove some unintelligible comments

## make.py

`make.py` generates Rust source code:

- `python3 make.py ic` generates the the raw bindings found in `src/ic.rs`

- `python3 make.py lib` generates the Rust wrapper in `src/lib.rs`

# The useful icapp utility

TurboPFor has a useful utlilty called `icapp`. It parses text (and binary?) files and performs compression and decompression using every available function. The main part of the output looks like this:

```asciidoc
  E MB/s     size     ratio     D MB/s function integer size=32 bits (lz=lz4,1) unsorted -1 
 1781.21     647168  50.20%    8837.61   1:p4nenc32         TurboPFor                
 1836.46     647168  50.20%   26336.43   2:p4nenc128v32     TurboPForV               
 2239.92     645909  50.10%   28397.86   3:p4nenc256v32     TurboPFor256             
  920.97     729594  56.59%    3121.03   4:p4ndenc32        TurboPFor    delta       
  951.02     729594  56.59%    5308.80   5:p4ndenc128v32    TurboPForV   delta       
 1206.38     728054  56.47%    6623.52   6:p4ndenc256v32    TurboPFor256 delta       
  870.71     850501  65.97%    2929.67   7:p4nd1enc32       TurboPFor    delta1      
...
```

The utility also prints histrograms that show the distribution of "max bits" in the input. "Max bits" is the highest non-zero bit in the input value.

```asciidoc
file: max bits histogram:
16:#################################################################################################### 100% 
file: delta max bits histogram:
00:############ 12% 
01:### 3.2% 
02:###### 6.4% 
03:########### 11% 
04:################# 17% 
05:#################### 20% 
06:######################## 24% 
07:#### 4.4% 
08:## 1.7% 
09: 0.1% 
10: 0.0001% 
17: 0.0001% 
```

Caveat: The very compact, really strangely formatted, sparsely documented 2000 line source code [icapp.c](https://github.com/powturbo/TurboPFor-Integer-Compression/blob/master/icapp.c) seems to be work in progress.

The caveat nonwithstanding, here are a few usage tips:

- Turn on maximum verbosity `-v5` to see the  parse result of the first 100 values

- Unless the `-f` flag is used (see below) the input is converted to integers. For example `./icapp -v5 -Ft.4 floats.txt` reads the text file `floats.txt` and transforms the input floats into integers by multiplying with 10000.

- The `-f4` `-f8` flag switches on float mode. For example `./icapp -v5 -Ft.4 -f4 floats.txt` reads the text file `float.txt` as single floats with 4 decimals. 

# Notes

Some notes on the original [turbopfor](https://github.com/powturbo/TurboPFor-Integer-Compression) library that were collected while writing the wrapper.

## Function name convention

The  library uses a compact naming scheme for its enc/dec/pack/unpack functions:

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

## Available Functions

The following width combinations are available:

| enc/dec | 8   | 16  | 32  | 64  |
| ------- |:---:|:---:|:---:|:---:|
| -       | x   | x   | x   | x   |
| 128v    |     | x   | x   |     |
| 256v    |     |     | x   |     |

Exceptions:

- enc/dec128v64

- in addition to enc/dec256v there is a 'w' version enc/dec256w

## Alignment, input and buffer sizes

Information about buffer sizes is scattered all over the place:

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
