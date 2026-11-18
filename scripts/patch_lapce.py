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

open(p, 'w').write(s)

print('=== patched floem/rfd lines ===')
for line in open(p):
    if 'floem' in line or 'rfd' in line:
        print(line, end='')