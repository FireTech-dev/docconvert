#!/usr/bin/env python3
"""USER/CI ONLY: measure and package an already-built binary. Never claims a passing release."""
import hashlib, json, pathlib, platform, sys, tarfile
profile = sys.argv[1]
if profile not in ('default', 'pdf-layout'): raise SystemExit('profile must be default or pdf-layout')
binary = pathlib.Path('target/release/docconvert' + ('.exe' if sys.platform == 'win32' else ''))
size = binary.stat().st_size
folder = pathlib.Path('release-artifacts'); folder.mkdir(exist_ok=True)
name = f'docconvert-{platform.system().lower()}-{platform.machine()}-{profile}'
archive = folder / (name + '.tar.gz')
with tarfile.open(archive, 'w:gz') as tar:
    for path in [binary, pathlib.Path('README.md'), pathlib.Path('docs/pdfium-setup.md'), pathlib.Path('Cargo.lock')]:
        tar.add(path, arcname=name + '/' + path.name)
report = dict(profile=profile, binary_bytes=size, default_budget_bytes=5_000_000,
              within_budget=profile != 'default' or size <= 5_000_000,
              archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
              lock_sha256=hashlib.sha256(pathlib.Path('Cargo.lock').read_bytes()).hexdigest(),
              native_library_bundled=False)
(folder / (name + '.json')).write_text(json.dumps(report, indent=2))
print(json.dumps(report, indent=2))
if not report['within_budget']: raise SystemExit('Default binary exceeds 5 MB release target; do not release')
