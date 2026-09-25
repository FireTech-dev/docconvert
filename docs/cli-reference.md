# CLI reference

Source candidate. The integration test `cli_docs` compares this flag inventory with
both profiles' live Clap help. User-run `--help` is authoritative for exact wrapping.

Usage: `docconvert [OPTIONS] [PATHS]...` — `--preview FILE` previews instead.
Output `-o` is a **directory**. Defaults → profile → config file → explicit CLI.
Boolean CLI switches come in both directions (`--flag` / `--no-flag`, plus
`--formulas`/`--comments`/`--strip-headers-footers` as the positive companions of
their `--no-*` defaults); explicit CLI wins, negation winning a direct conflict.
Config supports true/false for every key.

| Flag | Meaning |
| --- | --- |
| `--output` | `-o`: destination directory; recursive paths are mirrored |
| `--format` | `-f`: auto, md, txt |
| `--profile` | academic, technical, plain, spreadsheet, archive, accessibility, tts, ocr, strict |
| `--config` | Read key = value file |
| `--recursive` | `-r`: recurse without following symlinks |
| `--jobs` | Workers; 0 auto (maximum 8), explicit values capped at 256 |
| `--ocr` | off (default), auto, force; auto explicitly enables standalone-image OCR |
| `--ocr-lang` | Tesseract language argument, default eng; passed as one argument |
| `--ocr-command` | Trusted Tesseract-compatible executable, default tesseract; no shell |
| `--embed-assets` | Embed image data in Markdown |
| `--no-embed-assets` | Disable asset embedding (CLI override of profile/config) |
| `--include-hidden-sheets` | Include hidden workbook sheets |
| `--no-include-hidden-sheets` | Exclude hidden workbook sheets |
| `--no-formulas` | Omit formula lists |
| `--formulas` | Restore formula lists (CLI override) |
| `--no-comments` | Omit comments |
| `--comments` | Restore comments (CLI override) |
| `--describe-images` | Warn that descriptions are outside the offline core |
| `--no-describe-images` | Disable image-description warning |
| `--include-notes` | Include supported presentation notes |
| `--no-include-notes` | Exclude presentation notes |
| `--export-charts` | Request chart assets where supported; see format limitations |
| `--no-export-charts` | Disable chart-asset requests |
| `--no-strip-headers-footers` | Retain repeated PDF layout margin text; no effect in default build |
| `--strip-headers-footers` | Restore header/footer stripping (CLI override) |
| `--overwrite` | Replace existing outputs; never overwrite the source |
| `--no-overwrite` | Never replace existing outputs |
| `--strict` | Stop scheduling after first detected failure; in-flight files finish |
| `--no-strict` | Disable strict stop-on-failure |
| `--no-recursive` | Disable recursion |
| `--max-size` | Positive MiB input limit, default 200 |
| `--report` | none, text, json (default) |
| `--quiet` | `-q`: errors only |
| `--no-quiet` | Restore normal output |
| `--verbose` | `-v`: text reports on stderr |
| `--no-verbose` | Disable verbose reports |
| `--preview` | Inspect FILE without writing conversion outputs (OCR may use temporary files) |
| `--help` | `-h`: live help |
| `--version` | `-V`: package version |

## Layout-only flag

| Flag | Meaning |
| --- | --- |
| `--pdfium-lib-path` | Trusted PDFium library file or directory; pdf-layout feature only |

Exit statuses: **0** conversion/skip success, **1** conversion failure, **2** usage/config error.
PDFium environment errors discovered during conversion use exit 1, with Config diagnostics.
Config keys use the long flag spelling without `--`; profile, config, preview, help and
version are command-only. Layout library path is feature-gated in config too.
OCR auto degrades only when the executable is missing; a failing installed engine is
an error. Default-build PDFs cannot be rasterized even under force: a warning explains
that boundary. Preview can invoke OCR if explicitly enabled.
