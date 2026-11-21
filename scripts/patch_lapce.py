#!/usr/bin/env python3
"""Repoint lapce's floem deps at our adapted copies and drop rfd.

Runs inside the CI workspace after `git clone`ing lapce into ./lapce.
"""
import re

p = 'lapce/Cargo.toml'
s = open(p).read()

# floem -> ../floem (our adapted copy)
s = re.sub(
    r'\[workspace\.dependencies\.floem\]\n# path = "[^"]*"\ngit = "https://github\.com/lapce/floem"\nrev = "[^"]+"\n',
    '[workspace.dependencies.floem]\npath = "../floem"\n', s)

# floem-editor-core -> ../floem/editor-core
s = re.sub(
    r'\[workspace\.dependencies\.floem-editor-core\]\n# path = "[^"]*"\ngit = "https://github\.com/lapce/floem"\nrev = "[^"]+"\n',
    '[workspace.dependencies.floem-editor-core]\npath = "../floem/editor-core"\n', s)

# lapce enables a feature our floem does not ship: drop it.
s = s.replace(', "rfd-async-std"', '')

# Version skew: lapce pins lapce-xi-rope 0.3.2 but our floem (editor-core)
# uses 0.4.0, so lapce-core sees TWO copies of the crate and the rope types
# clash (E0308). Align lapce up to 0.4.0 -> single version in the graph.
s = s.replace(
    'lapce-xi-rope = { version = "0.3.2", features = ["serde"] }',
    'lapce-xi-rope = { version = "0.4.0", features = ["serde"] }')

open(p, 'w').write(s)

# ---------------------------------------------------------------------------
# wasi-experimental-http-wasmtime: wasmtime::Error does NOT implement
# std::error::Error on aarch64-linux-android, so the five
#   linker.func_wrap( ... )?;
# calls cannot convert into anyhow::Error. Map the error to a string first.
# The crate already uses `anyhow::format_err!`, so it is in scope.
q = 'wasi-experimental-http/crates/wasi-experimental-http-wasmtime/src/lib.rs'
w = open(q).read()
OLD = '        )?;'
NEW = ('        ).map_err(|e| '
       'anyhow::format_err!("wasi experimental http: {e}"))?;')
n = w.count(OLD)
w = w.replace(OLD, NEW)
open(q, 'w').write(w)
print(f'=== patched {n} func_wrap ? sites in wasi lib.rs ===')

print('=== patched floem/rfd/xi-rope lines ===')
for line in open(p):
    if 'floem' in line or 'rfd' in line or 'xi-rope' in line:
        print(line, end='')