#!/usr/bin/env python3
"""Prepare c2rust input for one libmpdec configuration.

Writes into WORKDIR:
  include/mpdecimal.h     generated from libmpdec/mpdecimal.h.in, with the
                          header config block taken verbatim from configure.ac
                          (used to compile the C tests)
  include-c2rust/mpdecimal.h
                          the same header with the literal suffixes of the
                          CONFIG_32 block adjusted (see translation_header)
  compile_commands.json   one entry per library object listed in
                          libmpdec/Makefile.in (OBJS), compiled against the
                          neutral libc headers in neutral-libc/ and
                          include-c2rust/
  src/libmpdec/           with --test-hooks: the library sources with
                          test-hooks/testit-hooks.patch applied (see
                          patched_sources), translated instead of libmpdec/

Everything is derived from the upstream build files so that re-running this
script always yields the same translation input.
"""

import argparse
import json
import re
import shlex
import shutil
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
LIBMPDEC = ROOT / "libmpdec"
NEUTRAL_LIBC = HERE / "neutral-libc"
TEST_HOOKS = HERE / "test-hooks" / "testit-hooks.patch"
HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")

# MACHINE -> (configure.ac header variable, MPD_CONFIG flags), see configure.ac
MACHINES = {
    "uint128": ("CONFIG_64", ["-DCONFIG_64", "-DANSI", "-DHAVE_UINT128_T"]),
    "ansi64": ("CONFIG_64", ["-DCONFIG_64", "-DANSI"]),
    "ansi32": ("CONFIG_32", ["-DCONFIG_32", "-DANSI"]),
}

# Library CFLAGS from configure.ac without warnings, -DNDEBUG and -O2.
# Optimization is dropped as recommended by c2rust; assertions are kept.
#
# -Dinline=: mpdecimal.c defines public functions as C99 `inline` next to the
# non-inline prototypes in mpdecimal.h, which makes them external definitions.
# c2rust 0.22.1 translates them without #[no_mangle], so the C tests fail to
# link (mpd_isnan, mpd_msword, ...). `inline` is only a hint otherwise, and
# system headers use `__inline`.
#
# -fmacro-prefix-map: __FILE__ (mpd_err_fatal) is relative to the repository.
BASE_CFLAGS = ["-std=c99", "-Dinline=", f"-fmacro-prefix-map={ROOT}/="]

# CONFIG_32 constants with the suffixes UL/L, while mpd_uint_t/mpd_ssize_t are
# uint32_t/int32_t (unsigned int/int).
SUFFIX_32 = re.compile(r"\b(\d+)(UL|L)\b")


def header_config(var):
    text = (ROOT / "configure.ac").read_text()
    m = re.search(r'^%s="(.*?)(?<!\\)"$' % re.escape(var), text, re.M | re.S)
    if not m:
        raise SystemExit(f"{var} not found in configure.ac")
    return m.group(1).replace('\\"', '"')


def translation_header(text):
    """Header for translation.

    The CONFIG_32 block spells mpd_uint_t constants with UL and mpd_ssize_t
    constants with L, while the CONFIG_64 block uses ULL/LL, the suffixes of
    the (neutral) uint64_t/int64_t. The translation of CONFIG_32 therefore
    casts through unsigned long/long where the translation of CONFIG_64 has
    none. The suffixes are replaced by U/none, which matches uint32_t/int32_t.
    CONFIG_32 requires a 32-bit size_t (see "#if MPD_SIZE_MAX !=
    MPD_UINT_MAX"), and on such targets long and int have the same width, so
    the values of all expressions are unchanged.
    """
    begin = text.index("/* BEGIN MPD_CONFIG_32 */")
    end = text.index("/* END MPD_CONFIG_32 */")
    block, n = SUFFIX_32.subn(lambda m: m.group(1) + ("U" if m.group(2) == "UL" else ""),
                              text[begin:end])
    if n == 0:
        raise SystemExit("no UL/L constants in the CONFIG_32 block")
    return text[:begin] + block + text[end:]


def apply_patch(patch, srcdir):
    """Apply a unified diff with paths a/libmpdec/FILE to the copies in
    srcdir. Context must match exactly. After each changed region a #line
    directive restores the line numbering of the unpatched file, so that
    __LINE__ (mpd_err_fatal) is the same as in the translation without the
    patch."""
    files = re.split(r"^--- a/libmpdec/(\S+)\n\+\+\+ b/libmpdec/\1\n", patch, flags=re.M)
    if files[0].strip() or len(files) < 3:
        raise SystemExit(f"{TEST_HOOKS}: unexpected patch format")
    for name, body in zip(files[1::2], files[2::2]):
        path = srcdir / name
        old = path.read_text().split("\n")
        new, pos = [], 0  # pos: index into old
        lines = body.split("\n")
        if lines[-1] == "":
            lines.pop()
        i = 0
        while i < len(lines):
            m = HUNK.match(lines[i])
            if not m:
                raise SystemExit(f"{name}: expected hunk header: {lines[i]!r}")
            start = int(m.group(1)) - 1
            new += old[pos:start]
            pos = start
            changed = False
            i += 1
            while i < len(lines) and not lines[i].startswith("@@"):
                tag, text = lines[i][:1], lines[i][1:]
                if tag in " -" and old[pos] != text:
                    raise SystemExit(f"{name}:{pos + 1}: context mismatch")
                if tag == " ":
                    if changed:
                        new.append(f"#line {pos + 1}")
                        changed = False
                    new.append(text)
                    pos += 1
                elif tag == "-":
                    pos += 1
                    changed = True
                elif tag == "+":
                    new.append(text)
                    changed = True
                else:
                    raise SystemExit(f"{name}: bad patch line {lines[i]!r}")
                i += 1
            if changed and pos < len(old) - 1:
                new.append(f"#line {pos + 1}")
        new += old[pos:]
        path.write_text("\n".join(new))


def patched_sources(work):
    """Copy the library sources and headers to WORK/src/libmpdec and apply
    the test hooks of mpdecimal-testit-2.5.1 (test-hooks/README.md)."""
    srcdir = work / "src" / "libmpdec"
    if srcdir.exists():
        shutil.rmtree(srcdir)
    srcdir.mkdir(parents=True)
    for f in sorted(LIBMPDEC.glob("*.[ch]")):
        shutil.copyfile(f, srcdir / f.name)
    apply_patch(TEST_HOOKS.read_text(), srcdir)
    return srcdir


def library_sources():
    text = (LIBMPDEC / "Makefile.in").read_text()
    m = re.search(r"^OBJS\s*:=\s*((?:.*\\\n)*.*)$", text, re.M)
    if not m:
        raise SystemExit("OBJS not found in libmpdec/Makefile.in")
    objs = m.group(1).replace("\\\n", " ").split()
    return [LIBMPDEC / (o[: -len(".o")] + ".c") for o in objs]


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--machine", required=True, choices=sorted(MACHINES))
    ap.add_argument("--workdir", required=True, type=Path)
    ap.add_argument("--cc", default="cc")
    ap.add_argument("--ndebug", action="store_true", help="add -DNDEBUG")
    ap.add_argument("--test-hooks", action="store_true",
                    help="translate the sources with test-hooks/testit-hooks.patch")
    ap.add_argument("--system-libc", action="store_true",
                    help="translate against the system headers and the "
                         "unmodified mpdecimal.h")
    ap.add_argument("clang_args", nargs="*", help="extra args, e.g. --target=")
    args = ap.parse_args()

    if (LIBMPDEC / "mpdecimal.h").exists():
        raise SystemExit("libmpdec/mpdecimal.h exists (in-tree configure?); "
                         "it would shadow the generated header")

    var, config_flags = MACHINES[args.machine]
    work = args.workdir.resolve()
    include = work / "include"
    include_c2rust = work / "include-c2rust"
    include.mkdir(parents=True, exist_ok=True)
    include_c2rust.mkdir(parents=True, exist_ok=True)

    template = (LIBMPDEC / "mpdecimal.h.in").read_text()
    header = template.replace("@MPD_HEADER_CONFIG@", header_config(var))
    (include / "mpdecimal.h").write_text(header)
    (include_c2rust / "mpdecimal.h").write_text(
        header if args.system_libc else translation_header(header))

    base = BASE_CFLAGS
    sources = library_sources()
    if args.test_hooks:
        # __FILE__ is libmpdec/FILE as without the patch
        srcdir = patched_sources(work)
        base = [f for f in BASE_CFLAGS if not f.startswith("-fmacro-prefix-map=")]
        base.append(f"-fmacro-prefix-map={srcdir.parent}/=")
        sources = [srcdir / s.name for s in sources]
    flags = base + config_flags + (["-DNDEBUG"] if args.ndebug else [])
    # compile_commands.json entries run in libmpdec/, so make relative include
    # paths given on the command line absolute.
    clang_args = [f"-I{Path(a[2:]).resolve()}" if a.startswith("-I") else a
                  for a in args.clang_args]
    neutral = [] if args.system_libc else [f"-I{NEUTRAL_LIBC}"]
    lib_flags = flags + neutral + [f"-I{include_c2rust}"] + clang_args

    def entry(src, flags):
        return {
            "directory": str(src.parent),
            "file": str(src),
            "arguments": [args.cc] + flags + ["-c", str(src)],
        }

    entries = [entry(src, lib_flags) for src in sources]
    (work / "compile_commands.json").write_text(json.dumps(entries, indent=2) + "\n")

    print(shlex.join([args.cc] + lib_flags))


if __name__ == "__main__":
    main()
