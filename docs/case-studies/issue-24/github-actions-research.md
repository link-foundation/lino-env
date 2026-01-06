# GitHub Actions Research: Job Dependencies and Skipped Jobs

## Key Findings from Online Research

### The Core Behavior

According to GitHub's documentation and community discussions, when a job in the `needs` dependency chain is skipped, all dependent jobs are also skipped unless they use a conditional expression that causes the job to continue.

**GitHub Documentation Quote:**
> "You can use the following status check functions as expressions in if conditionals. A default status check of success() is applied unless you include one of these functions."

This means without explicitly using `always()`, `failure()`, or `cancelled()`, the implicit `success()` check will cause the job to skip when dependencies are skipped.

### Known Issue

This behavior is documented in [GitHub Actions Runner Issue #491](https://github.com/actions/runner/issues/491) - "Job-level 'if' condition not evaluated correctly if job in 'needs' property is skipped". The issue has been acknowledged by GitHub but is in their backlog with no timeline for resolution.

### Recommended Workarounds

1. **Use `!failure()` instead of `success()`**
   - `if: "!failure()"` will run the job unless a dependency failed
   - This allows the job to run when dependencies are skipped

2. **Use `always()` with explicit conditions**
   - `if: always() && (needs.a.result == 'success' || needs.a.result == 'skipped')`
   - Forces evaluation of the `if` condition even when dependencies are skipped

3. **Combine `always()` with result checks**
   ```yaml
   if: |
     always() &&
     needs.job_a.result == 'success' &&
     (needs.job_b.result == 'success' || needs.job_b.result == 'skipped')
   ```

## Sources

- [GitHub Community Discussion #45058 - success() returns false if dependent jobs are skipped](https://github.com/orgs/community/discussions/45058)
- [GitHub Actions Runner Issue #2205 - Jobs skipped when NEEDS job ran successfully](https://github.com/actions/runner/issues/2205)
- [GitHub Actions Runner Issue #491 - Job-level "if" condition not evaluated correctly](https://github.com/actions/runner/issues/491)
- [GitHub Community Discussion #26945 - Jobs being skipped while using both needs and if](https://github.com/orgs/community/discussions/26945)
- [GitHub Docs - Using conditions to control job execution](https://docs.github.com/en/actions/using-jobs/using-conditions-to-control-job-execution)
- [GitHub Docs - Using jobs in a workflow](https://docs.github.com/actions/using-jobs/using-jobs-in-a-workflow)

## Application to Issue #24

The Rust workflow `lint` job has:
```yaml
needs: [detect-changes]
if: |
  github.event_name == 'push' ||
  github.event_name == 'workflow_dispatch' ||
  ...
```

When `detect-changes` is skipped (on `workflow_dispatch`), the `lint` job's `if` condition is never evaluated because the implicit `success()` check fails first.

The fix is to add `always()` to force the `if` condition evaluation:
```yaml
needs: [detect-changes]
if: |
  always() && (
    github.event_name == 'push' ||
    github.event_name == 'workflow_dispatch' ||
    ...
  )
```
