---
'lino-env': minor
---

Add dotenvx-like API with config(), get(), and set() functions

This release adds a new dotenvx-style API that makes lino-env easier to use:

- New `config()` function to load .lenv files into process.env
- New `get()` and `set()` helper functions for easy value access
- Default export for dotenvx-style usage: `import linoenv from 'lino-env'`
- Support for both ESM (import) and CommonJS (require) syntax
- Updated README with hero example similar to dotenvx

All existing APIs remain unchanged and fully backward compatible.
