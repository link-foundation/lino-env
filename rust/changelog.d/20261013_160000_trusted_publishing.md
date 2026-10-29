---
bump: patch
---

### Changed

- Prefer crates.io trusted publishing (OIDC) for automatic and manual instant releases, retaining `CARGO_REGISTRY_TOKEN` and `CARGO_TOKEN` secrets as fallbacks.
- Run release tooling on Node.js 24 to support its current dependencies.

### Fixed

- Preserve `cargo publish` failures when output is piped through `tee`, while continuing to accept already-published versions.
