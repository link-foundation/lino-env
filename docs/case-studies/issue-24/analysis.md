# Case Study: Issue #24 - Manual Release of Rust Package Didn't Work

## Issue Summary

When using `workflow_dispatch` to trigger a manual release of the Rust package, the release job was skipped instead of executing the version bump and crates.io publishing steps.

## Timeline of Events

### 2026-01-06 21:28:18 UTC - Manual Workflow Dispatch Triggered

- **Run ID:** 20762641678
- **Event:** `workflow_dispatch`
- **Trigger:** User manually triggered the Rust CI/CD Pipeline with bump_type option

### Workflow Execution Results

| Job | Status | Duration | Notes |
|-----|--------|----------|-------|
| Detect Changes | Skipped | 0s | Expected - job has `if: github.event_name != 'workflow_dispatch'` |
| Lint and Format Check | Skipped | 0s | **BUG** - Should have run but skipped due to dependency issue |
| Test (ubuntu-latest) | Success | 16s | Ran correctly with `always()` condition |
| Test (macos-latest) | Success | 19s | Ran correctly with `always()` condition |
| Test (windows-latest) | Success | 36s | Ran correctly with `always()` condition |
| Changelog Fragment Check | Skipped | 0s | Expected - only runs on PRs |
| Build Package | Skipped | 0s | **BUG** - Required lint to succeed, but lint was skipped |
| Auto Release | Skipped | 0s | Expected - only runs on push to main |
| Manual Release | Skipped | 0s | **BUG** - Never ran because build was skipped |

## Root Cause Analysis

### The Core Issue

The `lint` job depends on `detect-changes`, but `detect-changes` is skipped during `workflow_dispatch`. When a GitHub Actions job's dependency is skipped, the dependent job is also skipped by default, regardless of its own `if` condition.

### Dependency Chain Breakdown

```
detect-changes (skipped on workflow_dispatch)
       |
       v
     lint (skipped because dependency was skipped)
       |
       v
     build (skipped because lint.result != 'success')
       |
       v
manual-release (never runs because build was skipped)
```

### Relevant Code Locations

**1. detect-changes job (rust.yml:47-75)**
```yaml
detect-changes:
  name: Detect Changes
  runs-on: ubuntu-latest
  if: github.event_name != 'workflow_dispatch'  # Intentionally skipped
```

**2. lint job (rust.yml:112-155)**
```yaml
lint:
  name: Lint and Format Check
  runs-on: ubuntu-latest
  needs: [detect-changes]  # PROBLEM: Depends on skipped job
  if: |
    github.event_name == 'push' ||
    github.event_name == 'workflow_dispatch' ||  # This condition is never evaluated!
    ...
```

**3. build job (rust.yml:190-217)**
```yaml
build:
  name: Build Package
  runs-on: ubuntu-latest
  needs: [lint, test]
  if: always() && needs.lint.result == 'success' && needs.test.result == 'success'
  # When lint is skipped, needs.lint.result == 'skipped', not 'success'
```

**4. manual-release job (rust.yml:340-437)**
```yaml
manual-release:
  name: Manual Release
  needs: [lint, test, build]  # All three must succeed
  if: github.event_name == 'workflow_dispatch'
```

### Why Test Jobs Ran Successfully

The `test` job has a different pattern:
```yaml
test:
  needs: [detect-changes, changelog]
  if: always() && (github.event_name == 'push' || github.event_name == 'workflow_dispatch' || ...)
```

The `always()` function ensures the job's `if` condition is evaluated even when dependencies are skipped.

## Comparison with JavaScript Workflow

The JavaScript workflow (`js.yml`) handles manual releases differently:

```yaml
instant-release:
  name: Instant Release
  if: github.event_name == 'workflow_dispatch' && github.event.inputs.release_mode == 'instant'
  runs-on: ubuntu-latest
  # NO 'needs' - completely independent!
```

Key differences:
1. **No dependencies on CI jobs** - The `instant-release` job runs independently
2. **Two release modes** - Users can choose between "instant" (direct release) and "changeset-pr" (create PR for review)
3. **Self-contained steps** - All setup (Node.js, dependencies) is done within the job

## Proposed Solutions

### Solution 1: Add `always()` to lint job and update build condition (Minimal Fix)

```yaml
lint:
  name: Lint and Format Check
  runs-on: ubuntu-latest
  needs: [detect-changes]
  if: |
    always() && (
      github.event_name == 'push' ||
      github.event_name == 'workflow_dispatch' ||
      needs.detect-changes.outputs.rs-changed == 'true' ||
      ...
    )
```

And update `build` to handle skipped lint on workflow_dispatch:
```yaml
build:
  if: |
    always() && (
      (github.event_name == 'workflow_dispatch' && needs.test.result == 'success') ||
      (needs.lint.result == 'success' && needs.test.result == 'success')
    )
```

### Solution 2: Independent Manual Release Job (Match JavaScript Pattern)

Create a completely independent `manual-release` job that includes its own lint, test, and build steps - similar to the JavaScript `instant-release` job.

### Solution 3: Add Release Mode Options (Full JavaScript Parity)

Add `release_mode` input with options:
- `instant`: Direct release (includes lint/test/build in same job)
- `changeset-pr`: Creates a PR with changelog fragment for review

## Recommendation

**Combined Solution (Solution 1 + Solution 3)** was implemented:
1. Add `always() && !cancelled()` to job conditions to fix the skipped job issue
2. Add `release_mode` input with "instant" and "changelog-pr" options for full JavaScript parity

## Implementation Details

### Changes Made to `.github/workflows/rust.yml`

1. **Added `release_mode` workflow input** (lines 18-26):
   - `instant`: Direct release (includes lint/test/build verification)
   - `changelog-pr`: Creates a PR with changelog fragment for review

2. **Fixed `lint` job condition** (lines 116-128):
   - Added `always() && !cancelled()` to ensure the job runs even when `detect-changes` is skipped
   - Added comment explaining why `always()` is required

3. **Updated `build` job condition** (line 200):
   - Added `!cancelled()` for consistency

4. **Updated `auto-release` job condition** (lines 229-233):
   - Added `always() && !cancelled()` for consistent behavior
   - Added explicit check for `needs.build.result == 'success'`

5. **Renamed and updated `manual-release` job** (lines 357-461):
   - Renamed to "Instant Release"
   - Added `always() && !cancelled()` to condition
   - Added check for `github.event.inputs.release_mode == 'instant'`
   - Added check for `needs.build.result == 'success'`

6. **Added new `changelog-pr` job** (lines 463-533):
   - Creates a changelog fragment file
   - Opens a pull request using `peter-evans/create-pull-request@v7`
   - Matches the JavaScript workflow's `changeset-pr` pattern

## Files Affected

- `.github/workflows/rust.yml`
- `docs/case-studies/issue-24/analysis.md` (this file)
- `docs/case-studies/issue-24/github-actions-research.md`

## Related Links

- Issue: https://github.com/link-foundation/lino-env/issues/24
- Failed Run: https://github.com/link-foundation/lino-env/actions/runs/20762641678
- GitHub Actions Documentation: https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#jobsjob_idneeds
- GitHub Actions Runner Issue #491: https://github.com/actions/runner/issues/491
