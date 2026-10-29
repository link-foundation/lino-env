# lino-env

A library to operate .lenv files - an alternative to .env files that uses `: ` (colon-space) instead of `=` for key-value separation.

**Available in both JavaScript and Rust!**

## What are .lenv files?

.lenv files are configuration files similar to .env files, but with a different syntax:

```
# .env format (traditional)
GITHUB_TOKEN=gh_...
API_KEY=abc123

# .lenv format (this library)
GITHUB_TOKEN: gh_...
API_KEY: abc123
```

The key difference is the use of `: ` separator, which aligns with [links-notation](https://github.com/link-foundation/links-notation) format. If a key appears multiple times, the last value wins (rewrite semantics).

## Packages

| Package            | Language   | Directory | Status                                                                                  |
| ------------------ | ---------- | --------- | --------------------------------------------------------------------------------------- |
| [lino-env](./js)   | JavaScript | `./js`    | [![npm](https://img.shields.io/npm/v/lino-env)](https://www.npmjs.com/package/lino-env) |
| [lino-env](./rust) | Rust       | `./rust`  | [![crates.io](https://img.shields.io/crates/v/lino-env)](https://crates.io/crates/lino-env) |

## JavaScript Package

### Installation

```bash
npm install lino-env
```

### Quick Start

```bash
# create .lenv file
echo "HELLO: World" > .lenv

# create index.js
echo "import linoenv from 'lino-env'; linoenv.config(); console.log('Hello ' + process.env.HELLO)" > index.js

# run
node index.js
```

Output:

```
Hello World
```

### Usage

```javascript
import linoenv from 'lino-env';
linoenv.config();

console.log(`Hello ${process.env.HELLO}`);
```

For full API documentation, see the [JavaScript README](./js/README.md) (coming soon) or the source code.

## Rust Package

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lino-env = "0.1"
```

### Quick Start

```rust
use lino_env::LinoEnv;

// Create and write a new .lenv file
let mut env = LinoEnv::new(".lenv");
env.set("GITHUB_TOKEN", "gh_abc123");
env.set("API_KEY", "my_api_key");
env.write().unwrap();

// Read an existing .lenv file
let mut env = LinoEnv::new(".lenv");
env.read().unwrap();

// Get a value
if let Some(token) = env.get("GITHUB_TOKEN") {
    println!("Token: {}", token);
}
```

For full API documentation, see the [Rust README](./rust/README.md).

## File Format

.lenv files use the following format:

- Key-value separator: `: ` (colon followed by space)
- One key-value pair per line
- Empty lines and lines starting with `#` are ignored
- If a key appears multiple times, the last value wins (rewrite semantics)
- Values can contain spaces, colons, and other special characters

Example `.lenv` file:

```
# Configuration file
GITHUB_TOKEN: gh_abc123xyz
TELEGRAM_TOKEN: 054test456

# Values with special characters
URL: https://example.com:8080
MESSAGE: Hello World
```

## Repository Structure

```
lino-env/
├── js/                 # JavaScript package
│   ├── src/            # Source code
│   ├── tests/          # Tests
│   ├── .changeset/     # Changeset configuration
│   ├── package.json    # Package manifest
│   └── ...
├── rust/               # Rust package
│   ├── src/            # Source code
│   ├── tests/          # Tests (integration)
│   ├── changelog.d/    # Changelog fragments
│   ├── Cargo.toml      # Package manifest
│   └── ...
├── scripts/            # Shared scripts
└── .github/workflows/  # CI/CD workflows
    ├── js.yml          # JavaScript CI/CD
    └── rust.yml        # Rust CI/CD
```

## Development

### Prerequisites

- Node.js 20.x for JavaScript development
- Rust 1.70+ for Rust development

### Running Tests

```bash
# JavaScript tests
cd js && npm test

# Rust tests
cd rust && cargo test
```

### Linting and Formatting

```bash
# JavaScript
cd js && npm run lint && npm run format:check

# Rust
cd rust && cargo fmt --check && cargo clippy
```

### CI/CD Configuration

The repository uses GitHub Actions for automated testing and publishing.

**JavaScript Package:** Uses [NPM trusted publishing](https://docs.npmjs.com/trusted-publishers), which leverages OpenID Connect (OIDC) to enable secure publishing without long-lived tokens. This is configured on npmjs.com by adding the GitHub repository as a trusted publisher, and requires `id-token: write` permission in the workflow. No manual token configuration is needed in the repository.

**Rust Package:** Uses [crates.io trusted publishing](https://crates.io/docs/trusted-publishing) for automatic and manual instant releases. Register `link-foundation/lino-env` with the workflow filename `rust.yml` as a trusted publisher for the `lino-env` crate. These jobs do not use a GitHub environment, so leave that field unset. The publishing jobs request `id-token: write` and exchange the GitHub OIDC identity for a short-lived crates.io token.

After this workflow is merged, the migration can be completed from the repository root with `package-registry-manager setup --registry crates-io --migrate --execute` to register the trusted publisher on crates.io. Publisher registration requires a crate owner's authenticated session.

If trusted publishing is unavailable (including before registration), publishing falls back to `CARGO_REGISTRY_TOKEN`, then `CARGO_TOKEN`. Keep the existing secrets available as fallbacks. Tokens are passed through the publishing step's environment.

| Secret | Purpose | How to Obtain |
| ------ | ------- | ------------- |
| `CARGO_REGISTRY_TOKEN` | Preferred fallback for publishing the Rust package | [crates.io API tokens](https://crates.io/settings/tokens) |
| `CARGO_TOKEN` | Legacy fallback for publishing the Rust package | [crates.io API tokens](https://crates.io/settings/tokens) |

Release tooling for both packages runs on Node.js 24 because its dependencies require Node.js 22 or newer. JavaScript's library tests continue to run on Node.js 20, Bun and Deno. CI checks the release tool imports without publishing and tests Rust's publishing shell against successful uploads, existing versions, authentication errors and network failures.

Both packages are automatically published when changes are merged to the `main` branch, if the version has been bumped.

## License

This project is released into the public domain under [The Unlicense](http://unlicense.org).

## Repository

https://github.com/link-foundation/lino-env
