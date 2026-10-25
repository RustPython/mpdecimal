#!/usr/bin/env python3
"""Install the merged crate as the crate of the repository root.

usage: install.py CRATE

Writes Cargo.toml, rust-toolchain.toml, src/lib.rs and src/*.rs in the
repository root:

  - the modules of CRATE/src are installed as they are, except that the
    module paths crate::src::MOD become crate::MOD, since src/lib.rs
    declares the modules directly (c2rust's lib.rs declares them in a
    module src);
  - Cargo.toml names the package mpdecimal and declares the feature
    test-hooks, which selects the translation with the test hooks of
    testit (test-hooks/README.md).

The installed files are recorded with their SHA-256 in
c2rust/out/installed.sha256. The installation stops if an installed file was
changed since, or if a file of the root crate is not one installed here, so
that changes made in the root crate are not overwritten.
"""

import hashlib
import re
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
MANIFEST = HERE / "out" / "installed.sha256"

CARGO_TOML = """\
[package]
name = "mpdecimal"
version = "0.0.0"
edition = "2021"
publish = false
autobins = false
autoexamples = false
autotests = false
autobenches = false

[lib]
crate-type = ["staticlib", "rlib"]

[features]
# The test hooks of mpdecimal-testit (c2rust/test-hooks/README.md): extra
# symbols and fault injection used by the long tests. Never enable it
# outside of these tests.
test-hooks = []
"""

LIB_ATTRS = re.compile(r"^((?:#!\[.*\]\n)+)\npub mod src \{\n((?:    pub mod \w+;\n)+)\} // mod src\n$")


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_root():
    recorded = {}
    if MANIFEST.exists():
        for line in MANIFEST.read_text().splitlines():
            digest, name = line.split("  ", 1)
            recorded[name] = digest
    present = ["Cargo.toml", "rust-toolchain.toml"] + [
        str(p.relative_to(ROOT)) for p in sorted((ROOT / "src").glob("**/*")) if p.is_file()]
    for name in present:
        path = ROOT / name
        if not path.exists():
            continue
        if name not in recorded:
            raise SystemExit(f"{name} exists and was not installed by {Path(__file__).name}")
        if sha256(path) != recorded[name]:
            raise SystemExit(f"{name} was changed since the last installation")


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__.split("\n\n")[1])
    crate = Path(sys.argv[1])
    check_root()

    m = LIB_ATTRS.match((crate / "lib.rs").read_text())
    if not m:
        raise SystemExit(f"unexpected {crate / 'lib.rs'}")
    attrs, mods = m.groups()

    files = {
        "Cargo.toml": CARGO_TOML,
        "rust-toolchain.toml": (crate / "rust-toolchain.toml").read_text(),
        "src/lib.rs": attrs + "\n" + mods.replace("    pub mod", "pub mod"),
    }
    names = re.findall(r"pub mod (\w+);", mods)
    if sorted(names) != sorted(p.stem for p in (crate / "src").glob("*.rs")):
        raise SystemExit("lib.rs does not declare exactly the modules in src/")
    for name in names:
        text = (crate / "src" / f"{name}.rs").read_text()
        files[f"src/{name}.rs"] = text.replace("crate::src::", "crate::")
    if "crate::src" in "".join(files.values()):
        raise SystemExit("crate::src remains")

    if (ROOT / "src").exists():
        shutil.rmtree(ROOT / "src")
    (ROOT / "src").mkdir()
    manifest = []
    for name, text in files.items():
        path = ROOT / name
        path.write_text(text)
        manifest.append(f"{sha256(path)}  {name}")
    MANIFEST.write_text("\n".join(manifest) + "\n")
    print(f"installed {len(files)} files")


if __name__ == "__main__":
    main()
