#!/usr/bin/env python3

# %%
from pyparsing import *
import os
import sys
from typing import Tuple, Dict, List

convert_type = dict(
    size_t="usize",
    char = 'i8',
    short = 'i16',
    int = 'i32',
    unsignedchar = 'u8',
    unsignedshort = 'u16',
    unsignedint = 'u32',
    uint8_t = 'u8',
    uint16_t = 'u16',
    uint32_t = 'u32',
    uint64_t = 'u64',
)

ident = Word(alphas, alphanums + "_")
signed = one_of("signed unsigned")
vartype = Combine(
    Optional(signed) \
    + one_of("float double int char short long size_t uint8_t uint16_t uint32_t uint64_t") \
    + Optional(Word("*")), 
    adjacent=False
    )
arg_list = delimited_list(Group(vartype("type") + ident("name")))

function_decl = vartype("ret_type") + Optional(signed) + ident("name") + "(" + arg_list("args") + ")" + ";"

def parse_dir(headers_dir) -> List[Tuple[str, int, ParseResults]]:
    decls = []
    for header_file in os.listdir(headers_dir):
        if os.path.splitext(header_file)[1] == ".h":
            decls += parse_file(os.path.join(headers_dir, header_file))
    return decls

def parse_file(file_name) -> List[Tuple[str, int, ParseResults]]:
    fn_decls = []
    for line_no, line in enumerate(open(file_name)):
        line_no += 1
        line = line.strip()
        if line == "" or line[0:2] == "//":
            continue
        try:
            decl = function_decl.parseString(line)
            fn_decls.append((file_name, line_no, decl))
        except ParseException as e:
            print(f"Error in {file_name}, line {line_no}")
            print(e.explain())
            sys.exit(1)
    return fn_decls

def mk_rust_type(c_type, is_mut=False):
    if c_type[-2:] == "**":
        rust_type = convert_type.get(c_type[:-2])
        if rust_type is None:
            raise Exception(f"Cannot convert type** '{c_type[:-2]}'")
        if is_mut:
            rust_type = "**mut " + rust_type
        else:
            rust_type = "**const " + rust_type
    elif c_type[-1] == "*":
        rust_type = convert_type.get(c_type[:-1])
        if rust_type is None:
            raise Exception(f"Cannot convert type* '{c_type[:-1]}'")
        if is_mut:
            rust_type = "*mut " + rust_type
        else:
            rust_type = "*const " + rust_type
    else:
        rust_type = convert_type.get(c_type)
    return rust_type

def mk_rust_fn_arg(arg):
    """Turn `arg` into a Rust function argument varname: type"""
    rust_type = mk_rust_type(arg.type, is_mut=arg.name=='out')
    return f"{arg.name}: {rust_type}"

def print_fn(fn):
    print(f"{fn.name} -> {fn.ret_type}")
    for a in fn.args:
        print(f" - {a.name}: {a.type}")

def make_ic(decls: List[Tuple[str, int, ParseResults]]) -> str:
    """make the Rust bindings source code (i.e. content of src/ic.rs)"""

    rust_out = """\
//! Unsafe bindings auto-generated by make.py
#[link(name = "ic", kind = "static")]
extern "C" {
"""

    header_file = ''
    for this_header_file, line_no, fn in decls:
        if this_header_file != header_file:
            header_file = this_header_file
            rust_out += f"    // {header_file}\n"
        ret_type = mk_rust_type(fn.ret_type)
        if ret_type is None:
            raise Exception(f"Cannot convert type '{fn.ret_type}'")
        args = ', '.join(map(mk_rust_fn_arg, fn.args))
        rust_decl = f"    pub fn {fn.name}({args}) -> {ret_type};"
        rust_out += rust_decl + "\n"

    return rust_out + '} // extern "C"'

if __name__ == '__main__':
    
    # cmd line args housekeeping
    if len(sys.argv) >= 2 and sys.argv[1] in ['lib', 'ic']:
        mode = sys.argv[1]
    else:
        sys.stderr.write("I need the argument to be 'lib' or 'ic' ")
        sys.exit(1)

    decls = parse_dir("c_headers")

    if mode == 'ic':
        print(make_ic(decls))
    elif mode == 'lib':
        pass
    else:
        sys.stderr.write(f"Unknown mode '{mode}'")
