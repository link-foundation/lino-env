---
'lino-env': patch
---

Replace Jest with test-anywhere for testing

- Migrate from Jest to test-anywhere v0.4.0 for true cross-runtime testing
- Update all tests to use test-anywhere API (assert.ok, assert.equal, assert.deepEqual)
- Update CI to use native test commands: `node --test`, `bun test`, `deno test`
- Remove Jest configuration and dependencies (jest, babel-jest, @babel/preset-env)
- All 22 tests passing across Node.js, Bun, and Deno runtimes
