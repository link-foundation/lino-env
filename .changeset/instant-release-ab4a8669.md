---
'lino-env': patch
---

Replace manual release PR workflow with instant release mode

- Updated main.yml workflow to include instant release mode (default) and changeset-pr mode
- Removed separate manual-release.yml workflow file
- Instant release mode commits version bump directly to main branch without creating PR
- Added all necessary script files from js-ai-driven-development-pipeline-template
- Updated eslint.config.js to support Node.js 18+ globals
