#!/usr/bin/env python3
"""USER/CI ONLY: fetch a user-approved, SHA256-pinned PDFium archive; no runtime fetching."""
import hashlib, io, os, pathlib, tarfile, urllib.request, zipfile
url, expected = os.environ.get('PDFIUM_URL', ''), os.environ.get('PDFIUM_SHA256', '')
if not url.startswith('https://') or len(expected) != 64:
    raise SystemExit('Configure PDFIUM_URL and reviewed PDFIUM_SHA256 for this target.')
with urllib.request.urlopen(url, timeout=120) as response:
    data = response.read(250 * 1024 * 1024 + 1)
if len(data) > 250 * 1024 * 1024 or hashlib.sha256(data).hexdigest().lower() != expected.lower():
    raise SystemExit('PDFium archive size/checksum mismatch')
name = 'pdfium.dll' if os.name == 'nt' else ('libpdfium.dylib' if __import__('sys').platform == 'darwin' else 'libpdfium.so')
destination = pathlib.Path('native/pdfium'); destination.mkdir(parents=True, exist_ok=True)
if zipfile.is_zipfile(io.BytesIO(data)):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        members = [m for m in archive.infolist() if pathlib.PurePosixPath(m.filename).name == name and not m.is_dir()]
        if len(members) != 1 or members[0].file_size > 200 * 1024 * 1024: raise SystemExit('Expected exactly one bounded PDFium library')
        library = archive.read(members[0])
else:
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:*') as archive:
        members = [m for m in archive.getmembers() if pathlib.PurePosixPath(m.name).name == name and m.isfile()]
        if len(members) != 1 or members[0].size > 200 * 1024 * 1024: raise SystemExit('Expected exactly one bounded PDFium library')
        library = archive.extractfile(members[0]).read(200 * 1024 * 1024 + 1)
(destination / name).write_bytes(library)
print(destination.resolve())
