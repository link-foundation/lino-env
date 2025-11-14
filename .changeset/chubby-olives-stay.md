---
'lino-env': patch
---

Fix npm publish warnings and add missing ESLint dependency

- Fix repository URL format to include git+ prefix as required by npm
- Remove outdated Husky prepare script for v9 compatibility
- Initialize Husky v9 with pre-commit hook for lint-staged
- Add missing @eslint/js dependency required by eslint.config.js
