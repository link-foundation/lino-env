---
'lino-env': minor
---

Add Rust version of the code and restructure repository as monorepo

This release restructures the repository to support both JavaScript and Rust implementations:

- Moved all JS-specific code to ./js folder
- Added new Rust implementation in ./rust folder
- Created separate CI/CD workflows for each language (js.yml and rust.yml)
- The npm package functionality remains unchanged
