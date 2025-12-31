# lino-env (Rust)

A Rust library to read and write `.lenv` files.

## What is .lenv?

`.lenv` files are environment configuration files that use `: ` (colon-space) instead of `=` for key-value separation. This format is part of the links-notation specification.

Example `.lenv` file:

```
GITHUB_TOKEN: gh_abc123
TELEGRAM_TOKEN: 054xyz789
API_URL: https://api.example.com:8080
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lino-env = "0.1"
```

## Usage

### Basic Usage

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

### Multiple Values per Key

`.lenv` files support multiple values for the same key:

```rust
use lino_env::LinoEnv;

let mut env = LinoEnv::new(".lenv");

// Add multiple values for the same key
env.add("ALLOWED_HOST", "localhost");
env.add("ALLOWED_HOST", "example.com");
env.add("ALLOWED_HOST", "api.example.com");

// Get the last value
assert_eq!(env.get("ALLOWED_HOST"), Some("api.example.com".to_string()));

// Get all values
let hosts = env.get_all("ALLOWED_HOST");
assert_eq!(hosts, vec!["localhost", "example.com", "api.example.com"]);
```

### Convenience Functions

```rust
use lino_env::{read_lino_env, write_lino_env};
use std::collections::HashMap;

// Write using a HashMap
let mut data = HashMap::new();
data.insert("KEY1".to_string(), "value1".to_string());
data.insert("KEY2".to_string(), "value2".to_string());
write_lino_env(".lenv", &data).unwrap();

// Read into a LinoEnv instance
let env = read_lino_env(".lenv").unwrap();
println!("{:?}", env.get("KEY1"));
```

## API Reference

### LinoEnv

- `new(file_path)` - Create a new LinoEnv instance
- `read()` - Read and parse the .lenv file
- `write()` - Write the current data to the file
- `get(key)` - Get the last value for a key
- `get_all(key)` - Get all values for a key
- `set(key, value)` - Set a key to a single value (replaces all)
- `add(key, value)` - Add a value to a key (allows duplicates)
- `has(key)` - Check if a key exists
- `delete(key)` - Delete all values for a key
- `keys()` - Get all keys
- `to_hash_map()` - Convert to HashMap with last values

## License

Unlicense
