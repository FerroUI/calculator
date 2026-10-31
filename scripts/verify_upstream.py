#!/usr/bin/env python3
"""Check the complete pinned upstream snapshot, including C++ and original XAML."""
from pathlib import Path
import hashlib
import json
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
UP=ROOT/'upstream/calculator'
MANIFEST=ROOT/'docs/upstream-manifest.json'
if '--record' in sys.argv:
    paths=subprocess.check_output(['git','-C',str(UP),'ls-files','-z']).decode().strip('\0').split('\0')
    manifest={
        'repository':'https://github.com/microsoft/calculator',
        'revision':subprocess.check_output(['git','-C',str(UP),'rev-parse','HEAD']).decode().strip(),
        'files':{p:hashlib.sha256((UP/p).read_bytes()).hexdigest() for p in paths},
    }
    MANIFEST.write_text(json.dumps(manifest,indent=2)+'\n', encoding='utf-8')
else:
    manifest=json.loads(MANIFEST.read_text(encoding="utf-8"))
    failures=[]
    for p,sha in manifest['files'].items():
        path=UP/p
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest()!=sha:failures.append(p)
    if failures:raise SystemExit('Original source changed: '+', '.join(failures))
    if (UP/'.git').exists():
        revision=subprocess.check_output(['git','-C',str(UP),'rev-parse','HEAD']).decode().strip()
        if revision!=manifest['revision']:raise SystemExit('Unexpected upstream revision: '+revision)
    print(f"Verified {len(manifest['files'])} unchanged original files at {manifest['revision']}.")
