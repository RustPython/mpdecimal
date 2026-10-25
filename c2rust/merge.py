#!/usr/bin/env python3
"""Merge two crates into one crate, selecting variants with cfg attributes.

usage: merge.py CRATE_64 CRATE_32 OUT_CRATE
       merge.py --feature NAME CRATE CRATE_WITH_FEATURE OUT_CRATE

Both crates are split into top-level items (see normalize.split_items);
declarations in extern blocks are separate items. Items are identified by
their cfg attributes (from an earlier merge), kind and name. An item that is
equal in both crates is emitted once. Otherwise each variant is emitted with
the cfg attribute of its crate, which is added to the cfg attributes it
already has:

  - the normalized CONFIG_64 and CONFIG_32 crates:
    #[cfg(target_pointer_width = "64")] and
    #[cfg(target_pointer_width = "32")], the same selection that CPython
    makes between CONFIG_64 and CONFIG_32 (sizeof(size_t));
  - with --feature: #[cfg(not(feature = "NAME"))] and
    #[cfg(feature = "NAME")].

Items keep the order of the first crate; items only in the second crate
follow the item that precedes them there.
"""

import re
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from normalize import ITEM, split_items  # noqa: E402

WIDTH_CFG = ('#[cfg(target_pointer_width = "64")]', '#[cfg(target_pointer_width = "32")]')


def feature_cfg(name):
    return (f'#[cfg(not(feature = "{name}"))]', f'#[cfg(feature = "{name}")]')


def cfg_attrs(text):
    """The leading #[cfg] lines of an item or extern declaration."""
    attrs = []
    for line in text.split("\n"):
        if not DECL_ATTR.match(line):
            break
        if line.strip().startswith("#[cfg"):
            attrs.append(line.strip())
    return tuple(attrs)
DECL_ATTR = re.compile(r'^\s*#\[')


def extern_decls(block):
    """Split an extern "C" block into (key, text) per declaration."""
    lines = block.split("\n")
    if lines[0] != 'extern "C" {' or lines[-1] != "}":
        raise SystemExit(f"unexpected extern block start: {lines[0]!r}")
    decls, cur = [], []
    for line in lines[1:-1]:
        cur.append(line)
        if DECL_ATTR.match(line):
            continue
        if line.rstrip().endswith(";"):
            text = "\n".join(cur)
            m = ITEM.match(" ".join(l.strip() for l in cur if not DECL_ATTR.match(l)))
            if not m:
                raise SystemExit(f"unrecognized extern declaration: {text!r}")
            decls.append(((cfg_attrs(text), m.group(1).split()[0], m.group(2)), text))
            cur = []
    if cur:
        raise SystemExit(f"unterminated extern declaration: {cur!r}")
    return decls


def module_items(path):
    """Return the extern declarations and the items of a module, each a list
    of (key, text). c2rust emits at most one extern block, first."""
    externs, items = [], []
    for kind, name, text in split_items(path.read_text()):
        if kind == "other":
            if text.strip():
                raise SystemExit(f"{path}: unexpected top-level line {text!r}")
        elif kind == "extern":
            if items or externs:
                raise SystemExit(f"{path}: extern block is not the first item")
            externs = extern_decls(text)
        else:
            head = next(l for l in text.split("\n") if not l.startswith("#"))
            items.append(((cfg_attrs(text), ITEM.match(head).group(1).split()[0], name), text))
    for seq in (externs, items):
        keys = [k for k, _ in seq]
        dup = {k for k in keys if keys.count(k) > 1}
        if dup:
            raise SystemExit(f"{path}: duplicate items {sorted(dup)}")
    return externs, items


def merge_seq(a, b, cfg, indent=""):
    """Merge two lists of (key, text), cfg = (attribute for a, for b).
    Returns lines and statistics."""
    da, db = dict(a), dict(b)
    # items only in b, grouped by the last preceding key of b that is in a
    after = {None: []}
    anchor = None
    for k, _ in b:
        if k in da:
            anchor = k
        else:
            after.setdefault(anchor, []).append(k)
    stats = {"same": 0, "differ": 0, "only_a": 0, "only_b": 0}
    out = []

    def emit(which, text):
        out.append(indent + cfg[which])
        out.append(text)

    for k in after[None]:
        emit(1, db[k])
        stats["only_b"] += 1
    for k, text in a:
        if k not in db:
            emit(0, text)
            stats["only_a"] += 1
        elif db[k] == text:
            out.append(text)
            stats["same"] += 1
        else:
            emit(0, text)
            emit(1, db[k])
            stats["differ"] += 1
        for kb in after.get(k, []):
            emit(1, db[kb])
            stats["only_b"] += 1
    return out, stats


def main():
    args = sys.argv[1:]
    cfg = WIDTH_CFG
    if args[:1] == ["--feature"] and len(args) == 5:
        cfg = feature_cfg(args[1])
        args = args[2:]
    if len(args) != 3:
        raise SystemExit(__doc__.split("\n\n")[1])
    ca, cb, out = (Path(p) for p in args)
    for name in ("lib.rs", "Cargo.toml", "rust-toolchain.toml", "src/shim.rs"):
        if (ca / name).read_text() != (cb / name).read_text():
            raise SystemExit(f"{name} differs between {ca} and {cb}")
    mods = sorted(p.name for p in (ca / "src").glob("*.rs"))
    mods_b = sorted(p.name for p in (cb / "src").glob("*.rs"))
    if mods != mods_b:
        raise SystemExit(f"module lists differ: {mods} vs {mods_b}")

    if out.exists():
        shutil.rmtree(out)
    (out / "src").mkdir(parents=True)
    for name in ("lib.rs", "Cargo.toml", "rust-toolchain.toml", "src/shim.rs"):
        shutil.copyfile(ca / name, out / name)
    mods.remove("shim.rs")

    total = {}
    for mod in mods:
        ea, ia = module_items(ca / "src" / mod)
        eb, ib = module_items(cb / "src" / mod)
        ext_lines, ext_stats = merge_seq(ea, eb, cfg, indent="    ")
        item_lines, item_stats = merge_seq(ia, ib, cfg)
        if ext_lines:
            item_lines = ['extern "C" {'] + ext_lines + ["}"] + item_lines
        (out / "src" / mod).write_text("\n".join(item_lines) + "\n")
        for s in (ext_stats, item_stats):
            for k, v in s.items():
                total[k] = total.get(k, 0) + v
        print(f"{mod}: items {item_stats}, extern {ext_stats}")
    print(f"total: {total}")


if __name__ == "__main__":
    main()
