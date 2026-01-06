# lino-env

A library to operate .lenv files - an alternative to .env files that uses `: ` (colon-space) instead of `=` for key-value separation and supports duplicate keys.

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

The key difference is the use of `: ` separator, which aligns with [links-notation](https://github.com/link-foundation/links-notation) format. Additionally, .lenv files support duplicate keys, where multiple instances of the same key can exist.

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
- Duplicate keys are allowed
- Values can contain spaces, colons, and other special characters

Example `.lenv` file:

```
# Configuration file
GITHUB_TOKEN: gh_abc123xyz
TELEGRAM_TOKEN: 054test456

# Multiple servers
SERVER: server1.example.com
SERVER: server2.example.com

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

The repository uses GitHub Actions for automated testing and publishing. Maintainers need to configure the following secrets:

| Secret | Purpose | How to Obtain |
| ------ | ------- | ------------- |
| `NPM_TOKEN` | Publishing JavaScript package to npm | [npm access tokens](https://docs.npmjs.com/creating-and-viewing-access-tokens) |
| `CARGO_TOKEN` | Publishing Rust package to crates.io | [crates.io API tokens](https://crates.io/settings/tokens) |

Both packages are automatically published when changes are merged to the `main` branch, if the version has been bumped.

## License

This project is released into the public domain under [The Unlicense](http://unlicense.org).

## Repository

https://github.com/link-foundation/lino-env
