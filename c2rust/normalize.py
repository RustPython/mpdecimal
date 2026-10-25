#!/usr/bin/env python3
"""Normalize a translated crate in place.

usage: normalize.py CRATE_DIR {CONFIG_64|CONFIG_32}

The rules remove the differences between translations of the same
configuration for different targets that do not come from the C code, and
make the crate independent of the C type model of the translation target:

  - relooper block labels (current_block = <hash>) and freshN temporaries are
    renumbered per item in order of first appearance;
  - c_long/c_ulong become i64/u64 (CONFIG_64, translated for LP64) or
    i32/u32 (CONFIG_32, translated for ILP32), c_longlong/c_ulonglong become
    i64/u64; C ABI declarations (extern blocks and the definitions of
    functions whose prototype in mpdecimal.h uses long) keep c_long;
  - casts of integer literals through c_int to another type (N as c_int as
    T) become N as T: the value is unchanged if it compiles, as an integer
    literal that does not fit T is rejected by rustc. c2rust emits the
    intermediate c_int where the target type is a typedef of int (CONFIG_32)
    but not where it is wider (CONFIG_64);
  - casts to the type of their operand after the type mapping above (x as
    i64 as i64, c_long and the c_longlong of the typedef ptrdiff_t) are
    removed, since they are the identity;
  - CHAR_MAX becomes c_char::MAX, which follows the signedness of char of the
    target the crate is compiled for;
  - the opaque FILE, an extern type per module, becomes the FILE of the
    shim, so that the declarations of fprintf() etc. in the modules agree;
  - the hand-written shim (shim/shim.rs) is added as src/shim.rs, the
    no-op build.rs is removed, the #![feature] attributes are removed (no
    extern types remain and raw_ref_op is stable) and rust-toolchain.toml
    names the (stable) toolchain that builds the crates;
  - the modules are formatted with rustfmt of that toolchain, since the
    rules change the length of lines.

Every rule checks that it applied, so that a changed translation fails here
instead of silently passing through.
"""

import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
TOOLCHAIN = "1.99.0"
FEATURES = "#![feature(extern_types)]\n#![feature(raw_ref_op)]\n"
FILE_EXTERN = "    pub type mpd_c2rust_FILE;\n"
FILE_ALIAS = ("pub type FILE = mpd_c2rust_FILE;", "pub type FILE = crate::src::shim::FILE;")

CFFI = "::core::ffi::"
LONG_TYPES = {
    "CONFIG_64": {"c_long": "i64", "c_ulong": "u64"},
    "CONFIG_32": {"c_long": "i32", "c_ulong": "u32"},
}
LONGLONG_TYPES = {"c_longlong": "i64", "c_ulonglong": "u64"}

ITEM = re.compile(
    r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?'
    r'(fn|static(?:\s+mut)?|const|type|struct|union|enum|impl)\s+([A-Za-z_][A-Za-z0-9_]*)?')
LABEL_ASSIGN = re.compile(r'\bcurrent_block(?:_\d+)?\s*=\s*(\d+)\b')
NUMBER = re.compile(r'\b\d+\b')
FRESH = re.compile(r'\bfresh(\d+)\b')
INT_LITERAL_CAST = re.compile(r'\b(\d+) as ::core::ffi::c_int as (\w+)\b')
IDENTITY_CAST = re.compile(r'\bas (\w+) as \1\b')
LITERAL = re.compile(r'b?"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)\'')
CHAR_MAX = re.compile(
    r'^pub const CHAR_MAX: ::core::ffi::c_int = (?:SCHAR_MAX|UCHAR_MAX);$', re.M)


def split_items(text):
    """Split a rustfmt'ed file into chunks: (kind, name, text).

    kind is 'extern' for extern blocks, 'item' for top-level items with their
    attributes, 'other' for anything else (blank lines etc.)."""
    lines = text.split("\n")
    chunks, i, attrs = [], 0, []
    # extern blocks and items end with a line "}" or "...;" at depth 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("#[") or line.startswith("#!["):
            attrs.append(line)
            i += 1
            continue
        if line.startswith('extern "C" {'):
            j = i
            while lines[j] != "}":
                j += 1
            chunks.append(("extern", None, "\n".join(attrs + lines[i:j + 1])))
            attrs, i = [], j + 1
            continue
        m = ITEM.match(line)
        if not m:
            if attrs:
                raise SystemExit(f"attributes before unrecognized line: {line!r}")
            chunks.append(("other", None, line))
            i += 1
            continue
        j, depth = i, 0
        while True:
            code = LITERAL.sub("", lines[j])
            depth += code.count("{") - code.count("}")
            if depth == 0 and (code.rstrip().endswith(";") or code.startswith("}")):
                break
            j += 1
        chunks.append(("item", m.group(2), "\n".join(attrs + lines[i:j + 1])))
        attrs, i = [], j + 1
    return chunks


def renumber(text):
    labels = set(LABEL_ASSIGN.findall(text))
    if any(len(n) < 8 for n in labels):
        raise SystemExit(f"short relooper label in {text.splitlines()[0]!r}")
    if labels:
        order = {}
        for n in NUMBER.findall(text):
            if n in labels and n not in order:
                order[n] = str(len(order) + 1)
        text = NUMBER.sub(lambda m: order.get(m.group(0), m.group(0)), text)
    fresh = {}
    return FRESH.sub(lambda m: "fresh" + fresh.setdefault(m.group(1), str(len(fresh))), text)


def c_long_api():
    """Functions whose prototype in mpdecimal.h uses long (not long long)."""
    names = set()
    text = (ROOT / "libmpdec" / "mpdecimal.h.in").read_text()
    for proto in re.findall(r'^[A-Za-z_][^;#{}]*\([^;{}]*\);', text, re.M):
        if re.search(r'\blong\b(?!\s+long)', proto.replace("long long", "")):
            names.add(re.search(r'(\w+)\s*\(', proto).group(1))
    if names != {"mpd_radix"}:
        raise SystemExit(f"unexpected long API in mpdecimal.h: {sorted(names)}")
    return names


def map_types(text, config):
    types = {**LONG_TYPES[config], **LONGLONG_TYPES}
    return re.sub(r'::core::ffi::(c_u?long(?:long)?)\b', lambda m: types[m.group(1)], text)


def normalize_module(path, config, api, stats):
    source = path.read_text()
    chunks = split_items(source)
    if "\n".join(c[2] for c in chunks) != source:
        raise SystemExit(f"{path}: item split does not round-trip")
    out = []
    for kind, name, text in chunks:
        if kind == "extern" and FILE_EXTERN in text + "\n":
            text = (text + "\n").replace(FILE_EXTERN, "")[:-1]
            stats["file_extern"] += 1
        if kind == "item" and text == FILE_ALIAS[0]:
            text = FILE_ALIAS[1]
            stats["file_alias"] += 1
        if kind == "item":
            text = renumber(text)
            if name in api and f'extern "C" fn {name}(' in text:
                stats["api"] += 1
            else:
                text = map_types(text, config)
            text = INT_LITERAL_CAST.sub(r"\1 as \2", text)
            n = 1
            while n:
                text, n = IDENTITY_CAST.subn(r"as \1", text)
        out.append(text)
    text = "\n".join(out)
    text, n = CHAR_MAX.subn(
        "pub const CHAR_MAX: ::core::ffi::c_int = ::core::ffi::c_char::MAX as ::core::ffi::c_int;",
        text)
    stats["char_max"] += n
    path.write_text(text)


def main():
    if len(sys.argv) != 3 or sys.argv[2] not in LONG_TYPES:
        raise SystemExit(__doc__.split("\n\n")[1])
    crate, config = Path(sys.argv[1]), sys.argv[2]
    src = crate / "src"
    lib = crate / "lib.rs"

    if (src / "shim.rs").exists():
        raise SystemExit(f"{crate} is already normalized")

    api = c_long_api()
    stats = {"api": 0, "char_max": 0, "file_extern": 0, "file_alias": 0}
    modules = sorted(src.glob("*.rs"))
    if not modules:
        raise SystemExit(f"no modules in {src}")
    for path in modules:
        normalize_module(path, config, api, stats)
    if stats["api"] != len(api):
        raise SystemExit(f"C ABI functions with long: found {stats['api']} of {len(api)}")
    if stats["char_max"] == 0:
        raise SystemExit("CHAR_MAX not found")
    if stats["file_extern"] == 0 or stats["file_extern"] != stats["file_alias"]:
        raise SystemExit(f"FILE: {stats['file_extern']} extern types, "
                         f"{stats['file_alias']} aliases")

    text = lib.read_text()
    if text.count("pub mod src {\n") != 1 or text.count(FEATURES) != 1:
        raise SystemExit("unexpected lib.rs")
    text = text.replace(FEATURES, "")
    lib.write_text(text.replace("pub mod src {\n", "pub mod src {\n    pub mod shim;\n"))
    shutil.copyfile(HERE / "shim" / "shim.rs", src / "shim.rs")

    (crate / "build.rs").unlink()
    (crate / "rust-toolchain.toml").write_text(f'[toolchain]\nchannel = "{TOOLCHAIN}"\n')
    subprocess.run(["rustfmt", f"+{TOOLCHAIN}", "--edition", "2021"]
                   + [str(p) for p in sorted(src.glob("*.rs"))], check=True)


if __name__ == "__main__":
    main()
