# GitHub Actions `set-output` Deprecation Research

## Official Announcement

Source: [GitHub Blog Changelog - October 11, 2022](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)

## What's Being Deprecated

GitHub deprecated two workflow commands that are executed via stdout:
- `::save-state name={name}::{value}` - for saving state between workflow steps
- `::set-output name={name}::{value}` - for setting output values accessible by subsequent steps

## Replacement Solution

The new approach uses environment files instead of stdout commands:

### Old Approach (Deprecated)

```bash
# Setting output
echo "::set-output name=myOutput::myValue"

# Saving state
echo "::save-state name=myState::myValue"
```

### New Approach (Recommended)

```bash
# Setting output
echo "myOutput=myValue" >> $GITHUB_OUTPUT

# Saving state
echo "myState=myValue" >> $GITHUB_STATE
```

### JavaScript/Node.js Implementation

**Old approach:**
```javascript
console.log(`::set-output name=${key}::${value}`);
```

**New approach:**
```javascript
const fs = require('fs');
const outputFile = process.env.GITHUB_OUTPUT;
if (outputFile) {
  fs.appendFileSync(outputFile, `${key}=${value}\n`);
}
```

## Deprecation Timeline

| Date | Event |
|------|-------|
| October 11, 2022 | Initial deprecation announcement |
| May 31, 2023 | Originally planned full disablement |
| June 1, 2023 | Originally planned failure date for deprecated commands |
| July 24, 2023 | GitHub postponed removal due to "significant usage of these commands" |
| Present | Warnings shown but commands still functional |

## Requirements for Upgrade

1. **Action Authors**: Update `@actions/core` to v1.10.0 or later
2. **Self-hosted Runners**: Update to version 2.297.0 or greater

## Handling Multi-line Values

For multi-line values, use delimiters:

```bash
{name}<<{delimiter}
{value}
{delimiter}
```

Example:
```bash
echo "JSON_RESPONSE<<EOF" >> $GITHUB_OUTPUT
echo "$response_json" >> $GITHUB_OUTPUT
echo "EOF" >> $GITHUB_OUTPUT
```

## Why This Change?

The stdout-based commands were deprecated because:
1. **Security**: Environment files provide better isolation
2. **Reliability**: Reduces parsing issues with special characters
3. **Performance**: Avoids stdout parsing overhead

## Verification

To verify the fix works correctly:
1. Run a workflow that sets outputs
2. Check that no deprecation warnings appear in the logs
3. Verify that subsequent steps can still access the output values

## References

- [GitHub Blog Changelog](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)
- [GitHub Actions Workflow Commands Documentation](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions)
- [Environment Files Documentation](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#environment-files)
