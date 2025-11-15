# lino-env

## 0.2.5

### Patch Changes

- 5c3917b: Replace Jest with test-anywhere for testing
  - Migrate from Jest to test-anywhere v0.4.0 for true cross-runtime testing
  - Update all tests to use test-anywhere API (assert.ok, assert.equal, assert.deepEqual)
  - Update CI to use native test commands: `node --test`, `bun test`, `deno test`
  - Remove Jest configuration and dependencies (jest, babel-jest, @babel/preset-env)
  - All 22 tests passing across Node.js, Bun, and Deno runtimes

## 0.2.4

### Patch Changes

- a279fec: Test patch version

## 0.2.3

### Patch Changes

- e87a12d: Fix NPM publishing by upgrading npm CLI to support OIDC trusted publishing. This resolves authentication failures during automated releases by ensuring npm 11.5.1+ is available, which is required for OIDC-based authentication with GitHub Actions.

## 0.2.2

### Patch Changes

- 7bfef0d: Manual patch release

## 0.2.1

### Patch Changes

- d2e234e: Fix npm publish warnings and add missing ESLint dependency
  - Fix repository URL format to include git+ prefix as required by npm
  - Remove outdated Husky prepare script for v9 compatibility
  - Initialize Husky v9 with pre-commit hook for lint-staged
  - Add missing @eslint/js dependency required by eslint.config.js

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
