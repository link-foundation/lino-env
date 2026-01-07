# Case Study: GitHub Actions `set-output` Deprecation Warnings

## Issue Summary

- **Issue**: [#26 - These warnings must be fixed in CI/CD](https://github.com/link-foundation/lino-env/issues/26)
- **Type**: Bug
- **Status**: Open
- **Date Identified**: January 6, 2026

## Problem Description

The CI/CD pipeline generates 2 deprecation warnings during the "Instant Release" job:

```
The `set-output` command is deprecated and will be disabled soon. Please upgrade to using Environment Files.
For more information see: https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/
```

## Evidence

### CI Run Details

- **Run ID**: 20765751479
- **Workflow**: Rust CI/CD Pipeline
- **Job**: Instant Release
- **Head SHA**: e13ac13b192e0a2476e3cc15d3ada3b3c23905bc
- **Conclusion**: success (despite warnings)

### Warning Location in Logs

From `ci-logs/rust-20765751479.log` (lines 2510-2511):

```
Instant Release	UNKNOWN STEP	2026-01-06T23:51:14.1966757Z ##[warning]The `set-output` command is deprecated...
Instant Release	UNKNOWN STEP	2026-01-06T23:51:14.1982195Z ##[warning]The `set-output` command is deprecated...
```

The warnings appear after:
- "v0.1.1" version output
- "Tag v0.1.1 already exists" message

This indicates the warnings come from the `setOutput()` function calls in the version-and-commit.mjs script.

## Root Cause Analysis

### Source Code Location

File: `rust/scripts/version-and-commit.mjs`, lines 58-65:

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
  // Also log for visibility
  console.log(`::set-output name=${key}::${value}`);  // <-- DEPRECATED
}
```

### Technical Explanation

The `setOutput` function does two things:
1. **Correctly** writes to the `GITHUB_OUTPUT` environment file (new approach)
2. **Also** outputs the deprecated `::set-output` command to stdout (old approach)

While the new approach works correctly, the legacy stdout command is still being printed, causing GitHub Actions to emit deprecation warnings.

### Why Two Warnings?

Looking at the script, `setOutput` is called twice when a tag already exists:

```javascript
if (await checkTagExists(newVersion)) {
  console.log(`Tag v${newVersion} already exists`);
  setOutput('already_released', 'true');    // Warning #1
  setOutput('new_version', newVersion);     // Warning #2
  return;
}
```

## Comparison with JS Version

The JS version (`js/scripts/version-and-commit.mjs`, lines 103-108) is already correctly implemented:

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
  // No deprecated command - correctly omitted!
}
```

## Solution

Remove the deprecated `console.log` line from the Rust version's `setOutput` function.

### Before (deprecated)

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
  // Also log for visibility
  console.log(`::set-output name=${key}::${value}`);
}
```

### After (fixed)

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
    console.log(`Output: ${key}=${value}`);  // Optional: plain log for visibility
  }
}
```

## Timeline of Events

1. **October 11, 2022**: GitHub announces deprecation of `set-output` command
2. **May 31, 2023**: Originally planned full disablement
3. **July 24, 2023**: GitHub postpones removal due to significant usage
4. **January 6, 2026**: Issue #26 reported with warnings still appearing

## Related Resources

- [GitHub Changelog: Deprecating save-state and set-output commands](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)
- [GitHub Actions Environment Files documentation](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#environment-files)

## Impact Assessment

- **Severity**: Low (warnings only, workflow still succeeds)
- **Risk**: Medium (command may be disabled in future GitHub Actions updates)
- **Files Affected**: 1 (`rust/scripts/version-and-commit.mjs`)

## Lessons Learned

1. When implementing both old and new approaches for compatibility, ensure the old approach is eventually removed
2. Consistent patterns across language implementations (JS vs Rust scripts) help catch such discrepancies
3. CI warnings should be treated as actionable items before they become errors
