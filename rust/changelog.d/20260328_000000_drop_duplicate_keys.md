---
bump: major
---

### Changed

- Duplicate keys now use last-value-wins (rewrite semantics) instead of storing multiple values
- Internal data structure changed from `HashMap<String, Vec<String>>` to `HashMap<String, String>`

### Removed

- Removed `get_all()` method
- Removed `add()` method
