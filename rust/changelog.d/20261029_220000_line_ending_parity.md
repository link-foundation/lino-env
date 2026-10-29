---
bump: patch
---

### Fixed

- Verify existing CRLF support with regression tests for cross-platform buying preferences, whitespace, duplicate keys, quoted values, multiline blocks and read/write roundtrips. The Rust reader already recognizes LF and CRLF line endings.
- Keep the empty-file assertion compatible with current Clippy lints.
