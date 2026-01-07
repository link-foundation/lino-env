# Detailed Analysis: `set-output` Deprecation in Rust CI Pipeline

## Affected Files

### Primary Source

| File | Line | Issue |
|------|------|-------|
| `rust/scripts/version-and-commit.mjs` | 64 | Uses deprecated `::set-output` command |

### Comparison File

| File | Status |
|------|--------|
| `js/scripts/version-and-commit.mjs` | Already correctly implemented (no deprecated command) |

## Code Analysis

### Rust Version - `setOutput` Function

Location: `rust/scripts/version-and-commit.mjs:58-65`

```javascript
/**
 * Append to GitHub Actions output file
 * @param {string} key
 * @param {string} value
 */
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
  // Also log for visibility
  console.log(`::set-output name=${key}::${value}`);  // <-- LINE 64: DEPRECATED
}
```

### JS Version - `setOutput` Function (Correct Implementation)

Location: `js/scripts/version-and-commit.mjs:103-108`

```javascript
/**
 * Append to GitHub Actions output file
 * @param {string} key
 * @param {string} value
 */
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
}
```

## Call Sites Analysis

The `setOutput` function is called in the following locations in the Rust version:

### 1. When Tag Already Exists (lines 223-226)

```javascript
if (await checkTagExists(newVersion)) {
  console.log(`Tag v${newVersion} already exists`);
  setOutput('already_released', 'true');    // Call #1
  setOutput('new_version', newVersion);     // Call #2
  return;
}
```

### 2. When No Changes to Commit (lines 242-244)

```javascript
console.log('No changes to commit');
setOutput('version_committed', 'false');
setOutput('new_version', newVersion);
```

### 3. After Successful Push (lines 268-269)

```javascript
setOutput('version_committed', 'true');
setOutput('new_version', newVersion);
```

## CI Log Evidence

From `ci-logs/rust-20765751479.log`:

```
2508: Instant Release	UNKNOWN STEP	2026-01-06T23:51:14.1937150Z v0.1.1
2509: Instant Release	UNKNOWN STEP	2026-01-06T23:51:14.1945280Z Tag v0.1.1 already exists
2510: Instant Release	UNKNOWN STEP	##[warning]The `set-output` command is deprecated...
2511: Instant Release	UNKNOWN STEP	##[warning]The `set-output` command is deprecated...
2512: Instant Release	UNKNOWN STEP	##[group]Run cargo build --release
```

The warnings appear immediately after "Tag v0.1.1 already exists", confirming the issue is in the "tag already exists" code path (Call #1 and Call #2).

## Proposed Fix

### Option 1: Remove Deprecated Line (Minimal Change)

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
  }
}
```

### Option 2: Add Plain Log for Visibility (Recommended)

```javascript
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
    console.log(`Output: ${key}=${value}`);
  }
}
```

## Recommendation

Use **Option 2** to maintain visibility of what outputs are being set during workflow execution, which helps with debugging.

## Testing Strategy

1. Create a test script that simulates the `setOutput` function
2. Verify it writes to GITHUB_OUTPUT correctly
3. Ensure no `::set-output` appears in stdout
4. Run the actual workflow and verify no warnings appear

## Risk Assessment

| Factor | Assessment |
|--------|------------|
| Breaking Changes | None - output mechanism unchanged |
| Backwards Compatibility | Maintained - GITHUB_OUTPUT approach already implemented |
| Test Coverage | Manual CI run verification required |
