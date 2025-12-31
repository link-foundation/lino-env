# lino-env (JavaScript)

A JavaScript library to operate .lenv files - an alternative to .env files that uses `: ` (colon-space) instead of `=` for key-value separation and supports duplicate keys.

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

## Installation

```bash
npm install lino-env
```

## Quick Start

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

## Usage

### ESM (import)

```javascript
import linoenv from 'lino-env';
linoenv.config();

console.log(`Hello ${process.env.HELLO}`);
```

### CommonJS (require)

```javascript
require('lino-env').config();

console.log(`Hello ${process.env.HELLO}`);
```

## API Reference

### Dotenvx-like API

These functions provide a simple API similar to dotenvx for common use cases:

#### `config(options)`

Load .lenv file and inject into process.env

```javascript
import linoenv from 'lino-env';

linoenv.config(); // loads .lenv

// or specify a custom path
linoenv.config({ path: '.lenv.production' });
```

Returns: `{ parsed: Object }` - Object containing parsed key-value pairs

#### `get(key, options)`

Get a value from the loaded .lenv file

```javascript
import linoenv from 'lino-env';

const value = linoenv.get('API_KEY');

// or from a specific file
const value = linoenv.get('API_KEY', { path: '.lenv.production' });
```

Returns: `string | undefined`

#### `set(key, value, options)`

Set a value in a .lenv file

```javascript
import linoenv from 'lino-env';

linoenv.set('API_KEY', 'new_value');

// or to a specific file
linoenv.set('API_KEY', 'new_value', { path: '.lenv.production' });
```

### Class: LinoEnv

The main class for reading and writing .lenv files.

#### Constructor

```javascript
import { LinoEnv } from 'lino-env';

const env = new LinoEnv('.lenv');
```

#### Methods

##### `read()`

Reads and parses the .lenv file. If the file doesn't exist, initializes with empty data.

```javascript
const env = new LinoEnv('.lenv');
env.read();
```

Returns: `this` (for method chaining)

##### `write()`

Writes the current data back to the .lenv file.

```javascript
env.set('API_KEY', 'value').write();
```

Returns: `this` (for method chaining)

##### `get(reference)`

Gets the last instance of a reference (key).

```javascript
env.add('API_KEY', 'value1');
env.add('API_KEY', 'value2');
console.log(env.get('API_KEY')); // 'value2'
```

Returns: `string | undefined`

##### `getAll(reference)`

Gets all instances of a reference (key).

```javascript
env.add('API_KEY', 'value1');
env.add('API_KEY', 'value2');
console.log(env.getAll('API_KEY')); // ['value1', 'value2']
```

Returns: `string[]`

##### `set(reference, value)`

Sets a reference to a single value, replacing all existing instances.

```javascript
env.set('API_KEY', 'new_value');
```

Returns: `this` (for method chaining)

##### `add(reference, value)`

Adds a new instance of a reference, allowing duplicates.

```javascript
env.add('API_KEY', 'value1');
env.add('API_KEY', 'value2'); // Now there are 2 instances
```

Returns: `this` (for method chaining)

##### `has(reference)`

Checks if a reference exists.

```javascript
if (env.has('API_KEY')) {
  console.log('API_KEY exists');
}
```

Returns: `boolean`

##### `delete(reference)`

Deletes all instances of a reference.

```javascript
env.delete('API_KEY');
```

Returns: `this` (for method chaining)

##### `keys()`

Gets all keys.

```javascript
console.log(env.keys()); // ['GITHUB_TOKEN', 'API_KEY', ...]
```

Returns: `string[]`

##### `toObject()`

Converts to a plain object with the last instance of each key.

```javascript
env.add('KEY1', 'value1a');
env.add('KEY1', 'value1b');
env.set('KEY2', 'value2');

console.log(env.toObject());
// { KEY1: 'value1b', KEY2: 'value2' }
```

Returns: `Object`

### Convenience Functions

#### `readLinoEnv(filePath)`

Convenience function to read a .lenv file.

```javascript
import { readLinoEnv } from 'lino-env';

const env = readLinoEnv('.lenv');
console.log(env.get('GITHUB_TOKEN'));
```

Returns: `LinoEnv`

#### `writeLinoEnv(filePath, data)`

Convenience function to create and write a .lenv file from an object.

```javascript
import { writeLinoEnv } from 'lino-env';

writeLinoEnv('.lenv', {
  API_KEY: 'test_key',
  SECRET: 'test_secret',
});
```

Returns: `LinoEnv`

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

## License

This project is released into the public domain under [The Unlicense](http://unlicense.org).
