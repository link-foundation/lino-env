# Case Study: Issue #24 - Manual Release of Rust Package Didn't Work

## Quick Summary

**Problem:** Manual release of the Rust package via `workflow_dispatch` was being skipped instead of executing.

**Root Cause:** GitHub Actions job dependency chain issue - when a job with `needs: [other-job]` has its dependency skipped, the dependent job is also skipped, even if its own `if` condition would evaluate to true.

**Solution:** Added `always() && !cancelled()` to job conditions and implemented full parity with JavaScript workflow (instant and changelog-pr modes).

## Files in This Case Study

- [analysis.md](./analysis.md) - Detailed root cause analysis and solution documentation
- [github-actions-research.md](./github-actions-research.md) - Research findings from online sources

## Key Takeaways

1. **GitHub Actions `needs` behavior is subtle** - When a dependency is skipped, dependent jobs don't even evaluate their `if` condition
2. **Use `always()` to force condition evaluation** - This ensures the `if` condition is evaluated even when dependencies are skipped
3. **Use `!cancelled()` for safety** - Prevents jobs from running if the workflow was cancelled
4. **Check `needs.job.result` explicitly** - Ensure dependent jobs actually succeeded before proceeding

## Related Issues

- [GitHub Actions Runner Issue #491](https://github.com/actions/runner/issues/491) - Job-level "if" condition not evaluated correctly if job in "needs" property is skipped
