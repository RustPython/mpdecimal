#!/usr/bin/env python3
"""Deduplicate the definitions of a (merged) crate in place.

usage: dedupe.py CRATE

c2rust translates every C file into a module of its own, so each module
repeats the types, structs, constants and static inline functions of the
headers it includes, and calls functions of other modules through extern
blocks. This pass

  - moves items that are defined in more than one module, with the same
    definition in every module for each configuration (CONFIG_64 and
    CONFIG_32, with and without the feature of the test hooks, see
    merge.py), into src/common.rs, which every module glob-imports. Only types, structs, unions, constants and functions
    without #[no_mangle] (static inline functions of headers) are moved,
    never statics, since a static of each module is a separate object. An
    item is moved only together with all items it refers to;
  - replaces the declaration of a function or static in an extern block by a
    `use` of its #[no_mangle] definition in another module of the crate, so
    that calls between modules are type checked.
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from merge import WIDTH_CFG, extern_decls, feature_cfg  # noqa: E402
from normalize import ITEM, split_items  # noqa: E402

MOVABLE = {"type", "struct", "union", "const", "fn"}
IDENT = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\b")
GLOB = "use crate::src::common::*;"
FEATURE = "test-hooks"
# configurations: (pointer width, feature enabled)
CONFIGS = [(w, f) for w in (64, 32) for f in (False, True)]
CFG_TEST = {
    WIDTH_CFG[0]: lambda c: c[0] == 64,
    WIDTH_CFG[1]: lambda c: c[0] == 32,
    feature_cfg(FEATURE)[0]: lambda c: not c[1],
    feature_cfg(FEATURE)[1]: lambda c: c[1],
}
NO_MANGLE = re.compile(r'^#\[no_mangle\]\n(?:#\[.*\]\n)*'
                       r'pub (?:unsafe )?(?:extern "C" )?(?:fn|static(?: mut)?) (\w+)', re.M)


class Module:
    def __init__(self, path):
        self.path = path
        self.name = path.stem
        self.externs = []  # (name, kind, text)
        self.items = []    # (key, text), key = (cfg, kind, name)
        for kind, name, text in split_items(path.read_text()):
            if kind == "other":
                if text.strip():
                    raise SystemExit(f"{path}: unexpected top-level line {text!r}")
            elif kind == "extern":
                if self.items or self.externs:
                    raise SystemExit(f"{path}: extern block is not the first item")
                self.externs = [(k[2], k[1], t) for k, t in extern_decls(text)]
            else:
                lines = text.split("\n")
                cfg = tuple(l for l in lines if l.startswith("#[cfg"))
                head = next(l for l in lines if not l.startswith("#"))
                self.items.append(((cfg, ITEM.match(head).group(1).split()[0], name), text))


def configs_of(cfg, where):
    """The configurations in which an item with these cfg lines exists."""
    for attr in cfg:
        if attr not in CFG_TEST:
            raise SystemExit(f"{where}: unexpected cfg {attr}")
    return frozenset(c for c in CONFIGS if all(CFG_TEST[a](c) for a in cfg))


def cfg_lines(configs):
    """cfg attribute lines selecting exactly these configurations."""
    if configs == frozenset(CONFIGS):
        return []
    for attr in CFG_TEST:
        if configs == configs_of((attr,), attr):
            return [attr]
    terms = []
    for w, f in CONFIGS:
        if (w, f) in configs:
            feature = f'feature = "{FEATURE}"'
            terms.append(f'all(target_pointer_width = "{w}", '
                         f'{feature if f else f"not({feature})"})')
    return [f"#[cfg(any({', '.join(terms)}))]"]


def make_pub(text):
    lines = text.split("\n")
    i = next(i for i, l in enumerate(lines) if not l.startswith("#"))
    if not lines[i].startswith("pub "):
        lines[i] = "pub " + lines[i]
    return "\n".join(lines)


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__.split("\n\n")[1])
    crate = Path(sys.argv[1])
    src = crate / "src"
    if (src / "common.rs").exists():
        raise SystemExit(f"{crate} is already deduplicated")
    mods = [Module(p) for p in sorted(src.glob("*.rs")) if p.name != "shim.rs"]

    # #[no_mangle] definitions, including those of the shim
    defined = {}
    for path in sorted(src.glob("*.rs")):
        for name in NO_MANGLE.findall(path.read_text()):
            defined.setdefault(name, set()).add(path.stem)
    for name, where in defined.items():
        if len(where) > 1:
            raise SystemExit(f"#[no_mangle] {name} defined in {sorted(where)}")

    # per (kind, name) and configuration: the definitions (without cfg) in
    # all modules, and the modules that define it
    bodies = defaultdict(lambda: {c: set() for c in CONFIGS})
    where = defaultdict(set)
    order = []
    for m in mods:
        for (cfg, kind, name), text in m.items:
            body = "\n".join(l for l in text.split("\n") if not l.startswith("#[cfg"))
            for c in configs_of(cfg, f"{m.path}: {name}"):
                bodies[(kind, name)][c].add(body)
            where[(kind, name)].add(m.name)
            if (kind, name) not in order:
                order.append((kind, name))
    universe = {name for m in mods for (_, _, name), _ in m.items}
    universe |= {name for m in mods for name, _, _ in m.externs}
    cand = {
        k for k, b in bodies.items()
        if len(where[k]) > 1 and k[0] in MOVABLE
        and all(len(b[c]) <= 1 for c in CONFIGS)
        and not any("#[no_mangle]" in t.split("\n") for c in CONFIGS for t in b[c])
    }
    # closure: every crate item a candidate refers to must be a candidate
    refs = {
        k: {r for c in CONFIGS for t in bodies[k][c] for r in IDENT.findall(t)}
        & universe - {k[1]}
        for k in cand
    }
    while True:
        names = {name for _, name in cand}
        drop = {k for k in cand if not refs[k] <= names}
        if not drop:
            break
        cand -= drop
    common_names = {name for _, name in cand}

    # src/common.rs in order of first appearance
    common = []
    for k in order:
        if k not in cand:
            continue
        # one definition per distinct body, for the configurations that have it
        groups = {}
        for c in CONFIGS:
            for body in bodies[k][c]:
                groups.setdefault(body, set()).add(c)
        for body, configs in groups.items():
            common.append("\n".join(cfg_lines(frozenset(configs)) + [make_pub(body)]))
    (src / "common.rs").write_text("\n".join(common) + "\n")

    moved = replaced = 0
    for m in mods:
        uses, externs = [], []
        for name, kind, text in m.externs:
            home = defined.get(name)
            if home and kind in ("fn", "static") and m.name not in home:
                (home,) = home
                attrs = [l.strip() for l in text.split("\n") if l.strip().startswith("#[")]
                uses += attrs + [f"use crate::src::{home}::{name};"]
                replaced += 1
            else:
                externs.append(text)
        items = []
        for key, text in m.items:
            if key[1:] in cand:
                moved += 1
            else:
                items.append(text)
        body = "\n".join(items)
        head = []
        if set(IDENT.findall(body + "\n".join(externs))) & common_names:
            head.append(GLOB)
        head += uses
        if externs:
            head += ['extern "C" {'] + externs + ["}"]
        m.path.write_text("\n".join(head + items) + "\n")

    lib = crate / "lib.rs"
    text = lib.read_text()
    if text.count("    pub mod shim;\n") != 1:
        raise SystemExit("unexpected lib.rs")
    lib.write_text(text.replace("    pub mod shim;\n", "    pub mod common;\n    pub mod shim;\n"))
    print(f"common: {len(common)} items ({moved} copies removed), "
          f"extern declarations replaced by use: {replaced}")


if __name__ == "__main__":
    main()
