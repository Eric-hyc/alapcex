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
# std::error::Error on aarch64-linux-android不苟, so only the 5
# `linker.func_wrap(...)?` calls (returning wasmtime::Error, in add_to_linker)
# need the error mapped to anyhow. The memory.write/block_on calls in
# HostCalls/request return HttpError and must be left as plain `?`.
q = 'wasi-experimental-http/crates/wasi-experimental-http-wasmtime/src/lib.rs'
w = open(q).read()

# 1. Undo any previous over-broad .map_err replacement (both spellings).
for pat in (
    '        ).map_err(|e| anyhow::format_err!("wasi experimental http: {}", e))?;',
    '        ).map_err(|e| anyhow::format_err!("wasi experimental http: {e}"))?;',
):
    w = w.replace(pat, '        )?;')

# 2. Only the func_wrap sites: their closing `)?;` is preceded by a `},` line.
w = w.replace(
    '            },\n        )?;',
    '            },\n        ).map_err(|e| anyhow::format_err!("wasi experimental http: {}", e))?;')

# 3. func_wrap needs T: 'static on android.
w = w.replace(
    '    pub fn add_to_linker<T>(',
    "    pub fn add_to_linker<T: 'static>(")

open(q, 'w').write(w)
print(f'=== patched wasi lib.rs; func_wrap count={w.count("wasi experimental http")} ===')

print('=== patched floem/rfd/xi-rope lines ===')
for line in open(p):
    if 'floem' in line or 'rfd' in line or 'xi-rope' in line:
        print(line, end='')