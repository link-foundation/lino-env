---
'lino-env': patch
---

Recognize CRLF line endings without retaining carriage returns in configuration values. Preserve meaningful spaces, duplicate-key behavior and quoted values, including multiline blocks.

Update the Links Notation dependency to support releases from 0.25.2 through 0.27.x (locked to 0.27.1), with parser/formatter roundtrip tests and packed consumer installation checks that verify deduplication without overrides.

Run release tooling on Node.js 24 to support current command-stream dependencies, and verify release tool imports in CI.
