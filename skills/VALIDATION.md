# Skills pack static validation

Date: 2026-09-16. Documentation/file inspection only, not application execution.

- 34 actual skill files follow all six template sections in order.
- 42 detailed blueprint steps mapped; all 32 distinct detailed topics
  plus two architecture-only topics covered; no referenced topic omitted.
- Every skill has scoped steps, nonempty actionable guidance, anti-patterns, common
  mistakes, unchecked verification items and dated researched source links.
- Every source key resolves to retrieved, partial or explicitly search-only evidence.
- Relative skill/index links and referenced skill files exist; no template placeholders.
- All 8 supplied instruction/specification/template files match uploads byte-for-byte.
- All previous archive filenames remain present. 52 application/test/script/CI/
  manifest files compare byte-for-byte with revision 3: no functional code change.
- No application install/build/run/test/native/OCR/fuzz/CI operation was performed.

This review establishes file shape and coverage, not independent technical correctness,
full reading of every standard, resolved-version compatibility or source conformance.
Research limitations and unresolved decisions are explicitly documented in RESEARCH.md
and ARCHITECT-QUESTIONS.md. All application acceptance gates remain open.

## Skill file hashes

| File | SHA256 |
| --- | --- |
| `rust-core.md` | `85bd55f4e98d0b5fde3414e3aa091c64a3b0891ca830f1dd491d6e54d0de18b1` |
| `file-format-signatures.md` | `3b97d37be2e19db020a4dd741d6fdf58186c63e9f4940cd6f0614e422d3337fa` |
| `zip-container-anatomy.md` | `f2dfc0ea5244618b7dec82c61ef48e113f0b1b8bf352ce199fb5d6bdfb9a3c3f` |
| `parsing.md` | `2105c3b6bd2ddbd26cd36f50a9019c17756949498579844af27a7dc844c2f908` |
| `spreadsheet-data-modeling.md` | `828cb3bc6f3cf2e3cbf2ed8188e7e7d551079bcddbc3036bd373e781b6aebc9e` |
| `html-semantics.md` | `2105fd4243e13ba165f30fe8792fcd994d24f22a522b259e4094dcb651b9e8af` |
| `streaming-xml.md` | `132b71f25e0e838c7c82e2640f254f6d4c99e82fc2a0af2e2354fd533a80b14a` |
| `markdown-ascii-layout.md` | `2a938358b05549d47053cffec220d2b30d1ee690d60cc56f2732ae4bc87f47cc` |
| `cli-design.md` | `1fcf1f9aa0b7355e08aff44904ccc23b89b40139b1a9bf965bc8cb1c6bc2ef06` |
| `orchestration.md` | `2138ba64239d05431ff71a2c2d091fa630f84716354f2d7890dd23ec9635266b` |
| `ooxml.md` | `a485f7a273692ca7c1288286bdc66bd9b63ca3d35fcf84b6840995037cd2578d` |
| `ooxml-wordprocessingml.md` | `0267c02a1a6166ce642ee5f5dac82832fb992cfde2ef024757864a2acf2d3a39` |
| `epub-opf-packaging.md` | `abf563f1523b924c21e83c0e10bfe6ef7ffc19552d60d7febc36d78d9fe8e458` |
| `odf.md` | `390566996d166fb609d8143f1b1c35dab9643eb1cd83229cdd850ca0629571bf` |
| `rtf-control-word-grammar.md` | `b674877a0611f406880f8a04fa9ae54dc73782c9c1f458b5964cfc31768befd8` |
| `ooxml-spreadsheetml.md` | `de324383500319edc98be4d64da5685c6917353e438931b7b571b61b3f256bf0` |
| `ooxml-presentationml.md` | `39cbc866d9d3fd248cdcd2db71088d803574fcd23dccfd5cfc395c21643d8edc` |
| `integration-testing.md` | `d99c57141725f525511c82d429df01921cb135b76855e26b18a746f2f1a10ece` |
| `property-golden-testing.md` | `a900e5f67ac7eb6fba96e31fbf0fee03da50e060b69df7e01b3dd4e09e1f45f6` |
| `pdf-internals.md` | `10f875f2fdcd7f2599de3a7f152f21a7be71de0eb10e4a8d596da671a11a5c4a` |
| `text-heuristics.md` | `f8843265701304d937ce8f786284fbb60413651178e125a7710e2e45ce8f494a` |
| `ffi.md` | `93de23517cbe19988e25aaa2de8c9c50da76edecc606f42944458a25715870ec` |
| `pdf-layout-typography.md` | `283d1b274e7168f2c42047589d951dfff604cf3823aab81c4e8c5d98fe671032` |
| `unicode.md` | `91128ed34d046c31aaa965e17bde37bc4f803ed31343dfe2678efb86d4844d2e` |
| `image-header-formats.md` | `6237ff719a142c10996be94fbc56c1344069ac6a357f6db49cb72a9e73f40a53` |
| `process-spawning.md` | `e02b0b485a4d2d9502cdd6f4c726f1632bdfd5f16e3ae71ae51965ea4b62e887` |
| `temp-file-handling.md` | `41b8feff5dbcd11b83fb6b13955b7722731e147c3b8478b429ffd3909c822b42` |
| `batch-processing.md` | `c273621d802328531ca9d20a4eb8061558868962d4ad42a2129e5081133479ff` |
| `filesystem-traversal.md` | `5088fa8c2466ad0a59d73eead2ba85ef0c906dbd08c01c469e1ac0a6bafdbe5c` |
| `concurrency.md` | `6eaac677ed810aff3623ff9485ee1292cf8b4c73b7db5e98ead6ac73b3a26e04` |
| `security-testing.md` | `c42995112d5c128ddd9b67fcb04587ef6ba1999d282d82b56627718b80c6da92` |
| `docs.md` | `1da42af034348b8ea54384e65118c0a5daa6fd8c651f2aff1c7ce610709ecd9a` |
| `packaging.md` | `c33f983f6ad714580ac8363d8fb5c388f12b09cce4b43914fb2e5eaa2b2591f4` |
| `ci.md` | `e610e3d5988b267e76c050a1ec9a729578b63c336c0f1c869e98b77e37aa843d` |
