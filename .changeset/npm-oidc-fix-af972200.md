---
'lino-env': patch
---

Fix NPM publishing by upgrading npm CLI to support OIDC trusted publishing. This resolves authentication failures during automated releases by ensuring npm 11.5.1+ is available, which is required for OIDC-based authentication with GitHub Actions.
