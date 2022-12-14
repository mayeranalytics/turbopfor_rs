#!/usr/bin/env python3
import re
import sys
from collections import defaultdict
from make import vp4, bitpack, vsimple, vint

ic_preamble = """\
// auto-generated by make.py
// Here are the unsafe wrappers:
"""

lib_preamble = """\
// auto-generated by make.py
pub mod ic; // the unsafe wrappers
pub mod codec;
"""

RE1 = re.compile(r'size_t ([a-z0-9]+)\(.*\);.*')

algos1 = dict(
    p4 =  'turbopfor',
    bit = 'bit packing'
)

algos2 = dict(
    vb =  'variable byte',
    vs =  'variable simple',
)

fn_cats = {
    '':   'unsorted integer lists',
    'd':  'delta for increasing integer lists (sorted w/ duplicate)',
    'd1': 'delta for strictly increasing integer lists (sorted unique)',
    'f':  'sorted integer lists',
    'z':  'ZigZag for unsorted integer lists',
}

types = {
    '8': 8,
    '16': 16,
    '32': 32,
    '64': 64,
    '128v16': 16,
    '128v32': 32,
    '128v64': 64,
    '256v32': 32,
    '256w32': 32
}

codec_lookup = dict(
    enc = 'encode', dec = 'decode'
)

def indent(s:str, level=1) -> str:
    """Indent a block of rs code"""
    lines = [line.rstrip() for line in s.splitlines()]
    prefix = "\n"+" "*4*level
    return (prefix + prefix.join(lines))[1:] if len(lines) > 1 else " "*4*level + s.rstrip()

def unindent(s:str) -> str:
    """Unindent by the indent deduced from the first line"""
    lines = [line.rstrip() for line in s.splitlines()]
    n = len(lines[0]) - len(lines[0].lstrip(' '))
    spaces = ' '*n
    def cut(s):
        return s[n:] if s[:n] == spaces else s
    return "\n".join(map(cut, lines))

def make(algo, algo_name, fn_cat, fn_cat_desc, codec, size, typ):
    rs_fn_name = f"{fn_cat}{codec}{size}"
    ic_fn_name = f"{algo}n{fn_cat}{codec}{size}"
    if codec in ['enc', 'pack']:
        input_type = f'u{typ}'
        output_type = 'u8'
    elif codec in ['dec', 'unpack']:
        input_type = 'u8'
        output_type = f'u{typ}'
    else:
        sys.stderr.write("Unknown codec '{codec}'\n")
        sys,exit(1)
    codec_full_word = codec_lookup.get(codec, codec)
    # wrapper function definition
    wrap_def = f"""\
        pub fn {ic_fn_name}(inp: *const {input_type}, n: usize, out: *mut {output_type}) -> usize;\
        """
    # enc/pack and dec/unpack are different:
    if codec in ['enc', 'pack']:
        # pub enc/pack rs function
        fn_def = f"""\
            /// {algo_name.capitalize()} {codec_full_word} {fn_cat_desc} of `{input_type}`.
            /// # Arguments
            /// * `input` - `&[u{size}]` containing the uncompressed input
            /// * `output` - `&[u8]` containing the compressed output
            /// # Returns
            /// Number of bytes written to output
            pub fn {rs_fn_name}(input: &[{input_type}], output: &mut [{output_type}]) -> usize
            {{
                unsafe {{
                    ic::{ic_fn_name}(input.as_ptr(), input.len(), output.as_mut_ptr())
                }}
            }}\
            """ 
    elif codec in ['dec', 'unpack']:
        # pub dec/unpack rs function
        fn_def = f"""\
            /// {algo_name.capitalize()} {codec_full_word} {fn_cat_desc} into `{output_type}` list
            /// # Arguments
            /// * `input` - `[u8]` containing the compressed input
            /// * `output_len` - Length of decompressed data to be written to output
            /// * `output` - `&[{output_type}]` containing the decompressed output
            /// # Returns
            /// Number of bytes read from input
            pub fn {rs_fn_name}(input: &[{input_type}], output_len: usize, output: &mut [{output_type}]) -> usize
            {{
                unsafe {{
                    ic::{ic_fn_name}(input.as_ptr(), output_len, output.as_mut_ptr())
                }}
            }}\
            """
    else: # shouldn't happen
        sys.stderr.write(f"Unknown codec '{codec}'\n")
        sys.exit(1)

    return unindent(wrap_def), unindent(fn_def)     # for readablility the strings are indented - undo that with unindent


if __name__ == '__main__':
    
    # cmd line args housekeeping
    if len(sys.argv) < 2:
        sys.stderr.write("I need one argument: 'lib' or 'ic' ")
        sys.exit(1)
    lib_or_ic = sys.argv[1]
    if lib_or_ic not in ['lib', 'ic']:
        sys.stderr.write("I need the arguemtn to be 'lib' or 'ic' ")
        sys.exit(1)

    # parse the big fns string
    fn_names = set()
    for line_no, line in enumerate((vp4.header + bitpack.header).splitlines()):
        line = line.strip()
        if line == '': 
            continue
        m = RE1.match(line)
        if m is None:
            sys.stderr.write(f"Cannot parse line {line_no}: {line}\n")
            continue
        fn_names.add(m.group(1))

    # produce the rs code
    ic = ''
    lib = defaultdict(str)  # we separate the algos into different modules
    for algo, algo_name in algos1.items():
        for fn_cat, fn_cat_desc in fn_cats.items():
            for codec in ['enc', 'dec', 'pack', 'unpack']:
                for size, typ in types.items():
                    fn_name = f"{algo}n{fn_cat}{codec}{size}"
                    if fn_name in fn_names:
                        fn_names.remove(fn_name)
                        wrap_def, fn_def = make(algo, algo_name, fn_cat, fn_cat_desc, codec, size, typ)
                        ic += "    " + wrap_def + "\n"
                        lib[algo] += fn_def + "\n\n"

    # anything unaccounted for and we will stop with an error msg
    for fn_name in fn_names:
        print(f"{fn_name} not identified")
    if fn_names:
        sys.stderr.write("Sorry, yo must fix this.\n")
        sys.exit(1)

    # print the desired part
    if lib_or_ic == 'lib':
        ######### print lib.rs
        print(lib_preamble.strip())
        for algo, rs in lib.items():
            rs = rs.strip("\n")
            print("")
            print(unindent(f"""\
                pub mod {algo} {{

                    use crate::ic;

                {indent(rs)}

                }} // ends mod {algo}"""))
        print(unindent("""\
            #[cfg(test)]
            mod test;"""))
    else:
        ######### print ic.rs
        print(ic_preamble.strip())
        print(unindent("""\
            #[link(name = "ic", kind = "static")]
            extern "C" {"""))
        print(ic.strip("\n"))
        print("}")
