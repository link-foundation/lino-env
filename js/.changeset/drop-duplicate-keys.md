---
'lino-env': major
---

Drop support for multiple values of the same variable name. Duplicate keys now use last-value-wins (rewrite semantics). Removed `getAll()` and `add()` methods. Internal data structure changed from `Map<string, string[]>` to `Map<string, string>`.
