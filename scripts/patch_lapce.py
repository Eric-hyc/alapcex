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
# uses 0.4.0. Align up to 0.4.0 -> single version in the graph.
s = s.replace(
    'lapce-xi-rope = { version = "0.3.2", features = ["serde"] }',
    'lapce-xi-rope = { version = "0.4.0", features = ["serde"] }')

open(p, 'w').write(s)

# ---------------------------------------------------------------------------
# wasi-experimental-http-wasmtime: wasmtime::Error does NOT implement
# std::error::Error on aarch64-linux-android, so the 5 linker.func_wrap(...)?
# calls need the error mapped to anyhow.
q = 'wasi-experimental-http/crates/wasi-experimental-http-wasmtime/src/lib.rs'
w = open(q).read()
for pat in (
    '        ).map_err(|e| anyhow::format_err!("wasi experimental http: {}", e))?;',
    '        ).map_err(|e| anyhow::format_err!("wasi experimental http: {e}"))?;',
):
    w = w.replace(pat, '        )?;')
w = w.replace(
    '            },\n        )?;',
    '            },\n        ).map_err(|e| anyhow::format_err!("wasi experimental http: {}", e))?;')
w = w.replace(
    '    pub fn add_to_linker<T>(',
    "    pub fn add_to_linker<T: 'static>(")
open(q, 'w').write(w)

# ---------------------------------------------------------------------------
# trash: no Android backend. Keep the feature but adapt: on Android move the
# file into a `.trash/` dir next to it instead of calling the `trash` crate.
r = 'lapce/lapce-proxy/Cargo.toml'
t = open(r).read()
t = t.replace('trash = "3.0.6"\n', '')
t = t.rstrip('\n') + (
    '\n\n# trash has no Android backend; dispatch.rs adapts the call there\n'
    "[target.'cfg(not(target_os = \"android\"))'.dependencies.trash]\n"
    'version = "3.0.6"\n')
open(r, 'w').write(t)

d = 'lapce/lapce-proxy/src/dispatch.rs'
u = open(d).read()
OLD = (
    '            TrashPath { path } => {\n'
    '                let result = trash::delete(path)\n'
    '                    .map(|_| ProxyResponse::Success {})\n'
    '                    .map_err(|e| RpcError {\n'
    '                        code: 0,\n'
    '                        message: e.to_string(),\n'
    '                    });\n')
NEW = (
    '            TrashPath { path } => {\n'
    '                #[cfg(not(target_os = "android"))]\n'
    '                let result = trash::delete(path)\n'
    '                    .map(|_| ProxyResponse::Success {})\n'
    '                    .map_err(|e| RpcError {\n'
    '                        code: 0,\n'
    '                        message: e.to_string(),\n'
    '                    });\n'
    '                #[cfg(target_os = "android")]\n'
    '                let result = move_to_trash(&path)\n'
    '                    .map(|_| ProxyResponse::Success {})\n'
    '                    .map_err(|e| RpcError {\n'
    '                        code: 0,\n'
    '                        message: e.to_string(),\n'
    '                    });\n')
assert OLD in u, 'dispatch.rs TrashPath arm not found'
u = u.replace(OLD, NEW)
if 'fn move_to_trash' not in u:
    u += (
        '\n\n#[cfg(target_os = "android")]\n'
        'fn move_to_trash(path: &std::path::Path) -> std::io::Result<()> {\n'
        '    let Some(parent) = path.parent() else {\n'
        '        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "no parent dir"));\n'
        '    };\n'
        '    let Some(name) = path.file_name() else {\n'
        '        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "no file name"));\n'
        '    };\n'
        '    let trash_dir = parent.join(".trash");\n'
        '    std::fs::create_dir_all(&trash_dir)?;\n'
        '    std::fs::rename(path, trash_dir.join(name))?;\n'
        '    Ok(())\n'
        '}\n')
open(d, 'w').write(u)
print('=== patched trash (android move_to_trash) ===')

print('=== patched floem/rfd/xi-rope lines ===')
for line in open(p):
    if 'floem' in line or 'rfd' in line or 'xi-rope' in line:
        print(line, end='')