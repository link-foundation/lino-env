# lino-env

## 0.2.0

### Minor Changes

- cca9253: Add dotenvx-like API with config(), get(), and set() functions

  This release adds a new dotenvx-style API that makes lino-env easier to use:
  - New `config()` function to load .lenv files into process.env
  - New `get()` and `set()` helper functions for easy value access
  - Default export for dotenvx-style usage: `import linoenv from 'lino-env'`
  - Support for both ESM (import) and CommonJS (require) syntax
  - Updated README with hero example similar to dotenvx

  All existing APIs remain unchanged and fully backward compatible.

## 0.1.1

### Patch Changes

- 5f2ebcb: Prepare package for publication by adding links-notation dependency and multi-runtime CI/CD testing
  - Add links-notation ^0.11.2 as a dependency to ensure latest version is used
  - Set up GitHub Actions CI/CD workflow for testing across Bun, Deno, and Node.js runtimes
  - Configure changeset tooling for version management and releases
  - Add runtime-specific test configurations
