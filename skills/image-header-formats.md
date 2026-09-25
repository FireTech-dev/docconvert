# Skill: Image header formats

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P5-S01 actually need — not a general treatise on the topic.

## Scope

P5-S01 bounded dimension reading for PNG/JPEG/GIF/BMP/TIFF/WebP/JP2. Not complete image validation, orientation application, animation decoding or a new image dependency.

Blueprint/architecture aliases: `image header formats`, `Image headers`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Validate required signature/header structure before reading fields. Use checked offset+length arithmetic and checked integer conversions for every range. Dimensions must be positive and credible before any later pixel allocation; headers alone do not prove decodability.

2. PNG: verify the eight-byte signature and first IHDR framing; width/height are big-endian 32-bit values at offsets 16/20 from file start. GIF: validate GIF87a/GIF89a and read logical-screen little-endian 16-bit width/height at 6/8, not an arbitrary subframe size.

3. JPEG: walk markers from SOI using declared segment lengths; SOF contains precision, then big-endian height and width. Handle standalone markers separately, reject malformed lengths and stop before treating entropy-coded data as a header. The referenced libjpeg-turbo marker reader is implementation corroboration, not a dependency selection.

4. BMP: branch on DIB header size. BITMAPCOREHEADER and BITMAPINFOHEADER do not share dimension widths. For applicable signed-height DIBs, negative height means top-down; take absolute value with overflow checking. Do not reinterpret every negative width as valid.

5. TIFF 6.0: inspect II/MM byte order and magic, range-check the first IFD/count, and locate ImageWidth 256 / ImageLength 257 with supported SHORT or LONG values. Inline versus offset value storage follows type×count size. Bound IFD traversal/cycles; BigTIFF requires a separate declared capability.

6. WebP: validate RIFF/WEBP, bound chunks and odd-length padding. VP8X stores little-endian 24-bit canvas width-minus-one/height-minus-one; VP8 and VP8L use different frame-header layouts, so never reuse VP8X offsets.

7. JP2: walk bounded boxes, descend into jp2h, read ihdr height then width as big-endian fields. LBox includes the box header; 1 introduces XLBox, 0 extends to the enclosing end. Check parent bounds and recursion before advancing. Raw JPEG2000 codestreams are not JP2 box files.

8. A fixed small prefix may omit JPEG SOF or a TIFF IFD. Return dimensions unknown when a declared scan budget is exhausted; do not call all such files corrupt. Escalate the blueprint’s fixed-prefix versus offset-chasing tension.

## Not This

Do not decode pixels for this helper, allocate from header claims, accept truncated files as complete images, or treat EXIF display orientation as encoded width/height.

## Common Mistakes

RIFF padding omitted; JP2 length zero loops forever; TIFF offset overflows; BMP i32 minimum abs overflow; PNG four-byte prefix accepted as full signature validation.

## Verification Checklist

- [ ] All seven formats have complete known-dimension fixtures, plus truncated variants.
- [ ] TIFF/JP2 malicious offsets and lengths are bounded.
- [ ] Unknown dimensions preserve original asset bytes and produce the required warning.
- [ ] Separate tests cover VP8/VP8L/VP8X and both TIFF byte orders.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://www.w3.org/TR/png-3/) — Portable Network Graphics (PNG) Specification (Third Edition). Accessed 2026-09-16; retrieved. Evidence key: `png`.
- [2](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) — gif. Accessed 2026-09-16; retrieved. Evidence key: `gif`.
- [3](https://raw.githubusercontent.com/libjpeg-turbo/libjpeg-turbo/main/src/jdmarker.c) — jpeg. Accessed 2026-09-16; retrieved. Evidence key: `jpeg`.
- [4](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader) — BITMAPINFOHEADER (wingdi.h) - Win32 apps | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `bmp`.
- [2](https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf) — TIFF Revision 6.0 (1992), ITU-hosted copy. Accessed 2026-09-16; retrieved partial. Evidence key: `tiff`.
- [6](https://developers.google.com/speed/webp/docs/riff_container) — WebP Container Specification  |  Google for Developers. Accessed 2026-09-16; retrieved. Evidence key: `webp`.
- [7](https://www.loc.gov/preservation/digital/formats/fdd/fdd000143.shtml) — JPEG 2000 Part 1 (Core) jp2 File Format. Accessed 2026-09-16; retrieved. Evidence key: `jp2`.
- [8](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapcoreheader) — BITMAPCOREHEADER (wingdi.h) - Win32 apps | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `bmpcore`.
- [9](https://raw.githubusercontent.com/uclouvain/openjpeg/master/src/lib/openjp2/jp2.c) — jp2fields. Accessed 2026-09-16; retrieved. Evidence key: `jp2fields`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
