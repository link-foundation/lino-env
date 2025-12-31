# NPM Publishing Root Cause Analysis

## Issue

NPM publishing fails with error: `ENEEDAUTH This command requires you to be logged in to https://registry.npmjs.org`

## Investigation

### CI Logs Analysis

From run [19379315649](https://github.com/link-foundation/lino-env/actions/runs/19379315649/job/55454425315):

```
🦋  error an error occurred while publishing lino-env: ENEEDAUTH This command requires you to be logged in to https://registry.npmjs.org
🦋  error You need to authorize this machine using `npm adduser`
🦋  error npm error code ENEEDAUTH
🦋  error npm error need auth This command requires you to be logged in to https://registry.npmjs.org
```

### Environment Details

- Node.js: v20.19.5
- npm: 10.8.2 ⚠️ **This is the problem!**
- Workflow had `id-token: write` permission configured
- Workflow was setting `NODE_AUTH_TOKEN` and `NPM_TOKEN` environment variables

## Root Cause

The workflow was attempting to use NPM trusted publishing with OIDC, but **npm 11.5.1 or later is required** for OIDC support. The workflow was using npm 10.8.2, which does not support OIDC authentication.

Additionally:

1. The workflow was setting `NPM_TOKEN` environment variable, but this token was either missing or empty
2. npm 10.x cannot fall back to OIDC authentication even with `id-token: write` permission
3. Token-based authentication failed due to missing/invalid token

## Solution

Two changes were required:

### 1. Upgrade npm to support OIDC

Added a step to upgrade npm to the latest version (11.x+) which supports OIDC:

```yaml
- name: Upgrade npm for OIDC trusted publishing support
  run: npm install -g npm@latest
```

### 2. Remove token-based authentication

Removed the `NODE_AUTH_TOKEN` and `NPM_TOKEN` environment variables from the publish step, allowing npm to use OIDC authentication instead:

```yaml
- name: Publish to npm
  run: |
    # Publish to npm using OIDC trusted publishing (no token needed)
    npm run changeset:publish
  # No env: section with tokens
```

## NPM Trusted Publishing Requirements

Based on [npm documentation](https://docs.npmjs.com/trusted-publishers/):

### Prerequisites

1. npm CLI version 11.5.1 or later
2. GitHub Actions workflow with `id-token: write` permission
3. Package configured on npmjs.com with trusted publisher settings

### Configuration on npmjs.com

To enable trusted publishing, package maintainers need to:

1. Navigate to package settings on npmjs.com
2. Find the "Trusted Publisher" section
3. Select GitHub Actions as the CI/CD provider
4. Specify:
   - Organization/user name: `link-foundation`
   - Repository name: `lino-env`
   - Workflow filename: `main.yml`
   - Environment name: (optional, can be left empty)

### Benefits of OIDC Trusted Publishing

- No need to manage long-lived NPM tokens
- Automatic provenance attestations
- Better security through short-lived, workflow-specific credentials
- Eliminates risk of token exposure or leakage

## Testing

To test this fix:

1. Merge a changeset PR to main
2. The release workflow will trigger
3. npm will be upgraded to latest version (11.x+)
4. Publishing will use OIDC authentication automatically
5. Verify that publishing succeeds without authentication errors

## References

- [npm Trusted Publishers Documentation](https://docs.npmjs.com/trusted-publishers/)
- [GitHub Changelog: npm trusted publishing with OIDC is generally available](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/)
- [npm CLI v11.5.1 Release Notes](https://github.com/npm/cli/releases/tag/v11.5.1)
