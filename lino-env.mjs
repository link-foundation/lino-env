import { readFileSync, writeFileSync } from 'node:fs';

/**
 * LinoEnv - A library to read and write .lenv files
 * .lenv files use `: ` instead of `=` for key-value separation
 * Example: GITHUB_TOKEN: gh_....
 */
export class LinoEnv {
  constructor(filePath) {
    this.filePath = filePath;
    this.data = new Map();
  }

  /**
   * Read and parse the .lenv file
   * Stores all instances of each key (duplicates are allowed)
   */
  read() {
    try {
      const content = readFileSync(this.filePath, 'utf-8');
      this.data.clear();

      const lines = content.split('\n');
      for (const line of lines) {
        // Skip completely empty lines
        if (line.trim() === '' || line.trim().startsWith('#')) {
          continue;
        }

        // Parse line with `: ` separator
        const separatorIndex = line.indexOf(': ');
        if (separatorIndex === -1) {
          continue; // Skip malformed lines
        }

        const key = line.substring(0, separatorIndex).trim();
        const value = line.substring(separatorIndex + 2); // Don't trim the value to preserve spaces

        // Store all instances of the key
        if (!this.data.has(key)) {
          this.data.set(key, []);
        }
        this.data.get(key).push(value);
      }

      return this;
    } catch (_error) {
      // If file doesn't exist, initialize with empty data
      this.data.clear();
      return this;
    }
  }

  /**
   * Get the last instance of a reference (key)
   * @param {string} reference - The key to look up
   * @returns {string|undefined} The last value associated with the key
   */
  get(reference) {
    const values = this.data.get(reference);
    if (!values || values.length === 0) {
      return undefined;
    }
    return values[values.length - 1]; // Return last instance
  }

  /**
   * Get all instances of a reference (key)
   * @param {string} reference - The key to look up
   * @returns {string[]} All values associated with the key
   */
  getAll(reference) {
    return this.data.get(reference) || [];
  }

  /**
   * Set all instances of a reference to a new value
   * Replaces all existing instances with a single new value
   * @param {string} reference - The key to set
   * @param {string} value - The new value
   */
  set(reference, value) {
    this.data.set(reference, [value]);
    return this;
  }

  /**
   * Add a new instance of a reference (allows duplicates)
   * @param {string} reference - The key to add
   * @param {string} value - The value to add
   */
  add(reference, value) {
    if (!this.data.has(reference)) {
      this.data.set(reference, []);
    }
    this.data.get(reference).push(value);
    return this;
  }

  /**
   * Write the current data back to the .lenv file
   */
  write() {
    const lines = [];

    for (const [key, values] of this.data.entries()) {
      for (const value of values) {
        lines.push(`${key}: ${value}`);
      }
    }

    writeFileSync(this.filePath, `${lines.join('\n')}\n`, 'utf-8');
    return this;
  }

  /**
   * Check if a reference exists
   * @param {string} reference - The key to check
   * @returns {boolean}
   */
  has(reference) {
    return this.data.has(reference) && this.data.get(reference).length > 0;
  }

  /**
   * Delete all instances of a reference
   * @param {string} reference - The key to delete
   */
  delete(reference) {
    this.data.delete(reference);
    return this;
  }

  /**
   * Get all keys
   * @returns {string[]}
   */
  keys() {
    return Array.from(this.data.keys());
  }

  /**
   * Get all entries as an object (with last instance of each key)
   * @returns {Object}
   */
  toObject() {
    const obj = {};
    for (const [key, values] of this.data.entries()) {
      obj[key] = values[values.length - 1]; // Use last instance
    }
    return obj;
  }
}

/**
 * Convenience function to read a .lenv file
 * @param {string} filePath - Path to the .lenv file
 * @returns {LinoEnv}
 */
export function readLinoEnv(filePath) {
  return new LinoEnv(filePath).read();
}

/**
 * Convenience function to create and write a .lenv file
 * @param {string} filePath - Path to the .lenv file
 * @param {Object} data - Key-value pairs to write
 * @returns {LinoEnv}
 */
export function writeLinoEnv(filePath, data) {
  const env = new LinoEnv(filePath);
  for (const [key, value] of Object.entries(data)) {
    env.set(key, value);
  }
  env.write();
  return env;
}
