//! `LinoEnv` - A Rust library to read and write `.lenv` files.
//!
//! `.lenv` files use `: ` instead of `=` for key-value separation.
//! Example: `GITHUB_TOKEN: gh_....`
//!
//! If a key appears multiple times in a file, the last value wins (rewrite semantics).

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Package version (matches Cargo.toml version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn read_multiline_quoted_value(
    lines: &[&str],
    start_index: usize,
    value: &str,
) -> Option<(String, usize)> {
    let trimmed_value = value.trim_start();
    let quote = trimmed_value.chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }

    let first_part = &trimmed_value[quote.len_utf8()..];

    // Preserve existing single-line quoted value behavior.
    if first_part.contains(quote) {
        return None;
    }

    let mut parts = vec![first_part.to_string()];
    for (line_index, line) in lines.iter().enumerate().skip(start_index + 1) {
        if let Some(closing_quote_index) = line.find(quote) {
            parts.push(line[..closing_quote_index].to_string());
            return Some((parts.join("\n"), line_index));
        }

        parts.push((*line).to_string());
    }

    None
}

/// `LinoEnv` - A struct to read and write `.lenv` files.
///
/// `.lenv` files use `: ` instead of `=` for key-value separation.
///
/// # Examples
///
/// ```
/// use lino_env::LinoEnv;
/// use std::fs;
///
/// // Create a temporary file for testing
/// let path = std::env::temp_dir().join("test_lino_env_example.lenv");
/// let path = path.to_str().unwrap();
///
/// let mut env = LinoEnv::new(path);
/// env.set("GITHUB_TOKEN", "gh_test123");
/// env.set("TELEGRAM_TOKEN", "054test456");
/// env.write().unwrap();
///
/// // Read it back
/// let mut env2 = LinoEnv::new(path);
/// env2.read().unwrap();
/// assert_eq!(env2.get("GITHUB_TOKEN"), Some("gh_test123".to_string()));
///
/// // Clean up
/// fs::remove_file(path).ok();
/// ```
#[derive(Debug, Clone)]
pub struct LinoEnv {
    file_path: String,
    data: HashMap<String, String>,
}

impl LinoEnv {
    /// Create a new `LinoEnv` instance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the .lenv file
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let env = LinoEnv::new(".lenv");
    /// ```
    #[must_use]
    pub fn new<P: AsRef<str>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string(),
            data: HashMap::new(),
        }
    }

    /// Read and parse the .lenv file.
    ///
    /// If a key appears multiple times, the last value wins (rewrite semantics).
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// // Will return Ok even if file doesn't exist (data will be empty)
    /// let _ = env.read();
    /// ```
    pub fn read(&mut self) -> io::Result<&mut Self> {
        self.data.clear();

        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(self);
        }

        let content = fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut line_index = 0;
        while line_index < lines.len() {
            let line = lines[line_index];
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                line_index += 1;
                continue;
            }

            // Parse line with `: ` separator
            if let Some(separator_index) = line.find(": ") {
                let key = line[..separator_index].trim().to_string();
                let mut value = line[separator_index + 2..].to_string(); // Don't trim value to preserve spaces
                if let Some((multiline_value, next_line_index)) =
                    read_multiline_quoted_value(&lines, line_index, &value)
                {
                    value = multiline_value;
                    line_index = next_line_index;
                }

                // Last value wins (rewrite semantics)
                self.data.insert(key, value);
            }

            line_index += 1;
        }

        Ok(self)
    }

    /// Get the value of a reference (key).
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to look up
    ///
    /// # Returns
    ///
    /// The value associated with the key, or None if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY", "value");
    /// assert_eq!(env.get("KEY"), Some("value".to_string()));
    /// assert_eq!(env.get("NONEXISTENT"), None);
    /// ```
    #[must_use]
    pub fn get(&self, reference: &str) -> Option<String> {
        self.data.get(reference).cloned()
    }

    /// Set a reference to a value.
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to set
    /// * `value` - The new value
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY", "value");
    /// assert_eq!(env.get("KEY"), Some("value".to_string()));
    /// ```
    pub fn set(&mut self, reference: &str, value: &str) -> &mut Self {
        self.data.insert(reference.to_string(), value.to_string());
        self
    }

    /// Write the current data back to the .lenv file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// use std::fs;
    ///
    /// let path = std::env::temp_dir().join("test_lino_env_write.lenv");
    /// let path = path.to_str().unwrap();
    /// let mut env = LinoEnv::new(path);
    /// env.set("KEY", "value");
    /// env.write().unwrap();
    ///
    /// // Clean up
    /// fs::remove_file(path).ok();
    /// ```
    pub fn write(&self) -> io::Result<&Self> {
        let mut file = fs::File::create(&self.file_path)?;

        for (key, value) in &self.data {
            writeln!(file, "{key}: {value}")?;
        }

        Ok(self)
    }

    /// Check if a reference exists.
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY", "value");
    /// assert!(env.has("KEY"));
    /// assert!(!env.has("NONEXISTENT"));
    /// ```
    #[must_use]
    pub fn has(&self, reference: &str) -> bool {
        self.data.contains_key(reference)
    }

    /// Delete a reference.
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to delete
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY", "value");
    /// env.delete("KEY");
    /// assert!(!env.has("KEY"));
    /// ```
    pub fn delete(&mut self, reference: &str) -> &mut Self {
        self.data.remove(reference);
        self
    }

    /// Get all keys.
    ///
    /// # Returns
    ///
    /// A vector of all keys in the environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY1", "value1");
    /// env.set("KEY2", "value2");
    /// let keys = env.keys();
    /// assert!(keys.contains(&"KEY1".to_string()));
    /// assert!(keys.contains(&"KEY2".to_string()));
    /// ```
    #[must_use]
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Get all entries as a `HashMap`.
    ///
    /// # Returns
    ///
    /// A `HashMap` with each key mapped to its value.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.set("KEY1", "value1");
    /// env.set("KEY2", "value2");
    /// let obj = env.to_hash_map();
    /// assert_eq!(obj.get("KEY1"), Some(&"value1".to_string()));
    /// ```
    #[must_use]
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        self.data.clone()
    }
}

/// Convenience function to read a .lenv file.
///
/// # Arguments
///
/// * `file_path` - Path to the .lenv file
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Examples
///
/// ```
/// use lino_env::read_lino_env;
/// // Will work even if file doesn't exist
/// let env = read_lino_env(".lenv");
/// ```
pub fn read_lino_env<P: AsRef<str>>(file_path: P) -> io::Result<LinoEnv> {
    let mut env = LinoEnv::new(file_path);
    env.read()?;
    Ok(env)
}

/// Convenience function to create and write a .lenv file.
///
/// # Arguments
///
/// * `file_path` - Path to the .lenv file
/// * `data` - Key-value pairs to write
///
/// # Errors
///
/// Returns an error if the file cannot be written.
///
/// # Examples
///
/// ```
/// use lino_env::write_lino_env;
/// use std::collections::HashMap;
/// use std::fs;
///
/// let path = std::env::temp_dir().join("test_write_lino_env.lenv");
/// let path = path.to_str().unwrap();
/// let mut data = HashMap::new();
/// data.insert("KEY".to_string(), "value".to_string());
/// write_lino_env(path, &data).unwrap();
///
/// // Clean up
/// fs::remove_file(path).ok();
/// ```
#[allow(clippy::implicit_hasher)]
pub fn write_lino_env<P: AsRef<str>>(
    file_path: P,
    data: &HashMap<String, String>,
) -> io::Result<LinoEnv> {
    let mut env = LinoEnv::new(file_path);
    for (key, value) in data {
        env.set(key, value);
    }
    env.write()?;
    Ok(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn cleanup(path: &str) {
        fs::remove_file(path).ok();
    }

    fn test_file(name: &str) -> String {
        std::env::temp_dir()
            .join(format!("lino_env_test_{name}.lenv"))
            .to_string_lossy()
            .to_string()
    }

    mod basic_tests {
        use super::*;

        #[test]
        fn test_create_and_write() {
            let test_file = test_file("basic_create_write");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("GITHUB_TOKEN", "gh_test123");
            env.set("TELEGRAM_TOKEN", "054test456");
            env.write().unwrap();

            assert!(Path::new(&test_file).exists());
            cleanup(&test_file);
        }

        #[test]
        fn test_read() {
            let test_file = test_file("basic_read");
            cleanup(&test_file);
            // First create a file
            let mut env1 = LinoEnv::new(&test_file);
            env1.set("GITHUB_TOKEN", "gh_test123");
            env1.set("TELEGRAM_TOKEN", "054test456");
            env1.write().unwrap();

            // Then read it
            let mut env2 = LinoEnv::new(&test_file);
            env2.read().unwrap();

            assert_eq!(env2.get("GITHUB_TOKEN"), Some("gh_test123".to_string()));
            assert_eq!(env2.get("TELEGRAM_TOKEN"), Some("054test456".to_string()));
            cleanup(&test_file);
        }
    }

    mod get_tests {
        use super::*;

        #[test]
        fn test_get_value() {
            let test_file = test_file("get_value");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("API_KEY", "value1");

            assert_eq!(env.get("API_KEY"), Some("value1".to_string()));
            cleanup(&test_file);
        }

        #[test]
        fn test_get_nonexistent() {
            let test_file = test_file("get_nonexistent");
            let env = LinoEnv::new(&test_file);
            assert_eq!(env.get("NON_EXISTENT"), None);
        }
    }

    mod set_tests {
        use super::*;

        #[test]
        fn test_set_overwrites() {
            let test_file = test_file("set_overwrites");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("API_KEY", "value1");
            env.set("API_KEY", "new_value");

            assert_eq!(env.get("API_KEY"), Some("new_value".to_string()));
            cleanup(&test_file);
        }
    }

    mod duplicate_key_tests {
        use super::*;

        #[test]
        fn test_duplicate_keys_last_value_wins() {
            let test_file = test_file("duplicate_keys");
            cleanup(&test_file);
            // Write a file with duplicate keys manually
            fs::write(&test_file, "A: value1\nA: value2\n").unwrap();

            let mut env = LinoEnv::new(&test_file);
            env.read().unwrap();

            assert_eq!(env.get("A"), Some("value2".to_string()));
            cleanup(&test_file);
        }
    }

    mod has_tests {
        use super::*;

        #[test]
        fn test_has_existing() {
            let test_file = test_file("has_existing");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("KEY", "value");

            assert!(env.has("KEY"));
            cleanup(&test_file);
        }

        #[test]
        fn test_has_nonexistent() {
            let test_file = test_file("has_nonexistent");
            let env = LinoEnv::new(&test_file);
            assert!(!env.has("NON_EXISTENT"));
        }
    }

    mod delete_tests {
        use super::*;

        #[test]
        fn test_delete() {
            let test_file = test_file("delete");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("KEY", "value1");
            env.delete("KEY");

            assert!(!env.has("KEY"));
            assert_eq!(env.get("KEY"), None);
            cleanup(&test_file);
        }
    }

    mod keys_tests {
        use super::*;

        #[test]
        fn test_keys() {
            let test_file = test_file("keys");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("KEY1", "value1");
            env.set("KEY2", "value2");
            env.set("KEY3", "value3");

            let keys = env.keys();
            assert!(keys.contains(&"KEY1".to_string()));
            assert!(keys.contains(&"KEY2".to_string()));
            assert!(keys.contains(&"KEY3".to_string()));
            assert_eq!(keys.len(), 3);
            cleanup(&test_file);
        }
    }

    mod to_hash_map_tests {
        use super::*;

        #[test]
        fn test_to_hash_map() {
            let test_file = test_file("to_hash_map");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.set("KEY1", "value1");
            env.set("KEY2", "value2");

            let obj = env.to_hash_map();
            assert_eq!(obj.get("KEY1"), Some(&"value1".to_string()));
            assert_eq!(obj.get("KEY2"), Some(&"value2".to_string()));
            cleanup(&test_file);
        }
    }

    mod persistence_tests {
        use super::*;

        #[test]
        fn test_persist_values() {
            let test_file = test_file("persist_values");
            cleanup(&test_file);
            let mut env1 = LinoEnv::new(&test_file);
            env1.set("KEY", "value");
            env1.write().unwrap();

            let mut env2 = LinoEnv::new(&test_file);
            env2.read().unwrap();

            assert_eq!(env2.get("KEY"), Some("value".to_string()));
            cleanup(&test_file);
        }
    }

    mod convenience_function_tests {
        use super::*;

        #[test]
        fn test_read_lino_env() {
            let test_file_path = test_file("convenience_read");
            cleanup(&test_file_path);
            let mut data = HashMap::new();
            data.insert("GITHUB_TOKEN".to_string(), "gh_test".to_string());
            data.insert("TELEGRAM_TOKEN".to_string(), "054test".to_string());
            write_lino_env(&test_file_path, &data).unwrap();

            let env = read_lino_env(&test_file_path).unwrap();
            assert_eq!(env.get("GITHUB_TOKEN"), Some("gh_test".to_string()));
            assert_eq!(env.get("TELEGRAM_TOKEN"), Some("054test".to_string()));
            cleanup(&test_file_path);
        }

        #[test]
        fn test_write_lino_env() {
            let test_file_path = test_file("convenience_write");
            cleanup(&test_file_path);
            let mut data = HashMap::new();
            data.insert("API_KEY".to_string(), "test_key".to_string());
            data.insert("SECRET".to_string(), "test_secret".to_string());
            write_lino_env(&test_file_path, &data).unwrap();

            let env = read_lino_env(&test_file_path).unwrap();
            assert_eq!(env.get("API_KEY"), Some("test_key".to_string()));
            assert_eq!(env.get("SECRET"), Some("test_secret".to_string()));
            cleanup(&test_file_path);
        }
    }

    mod format_tests {
        use super::*;

        #[test]
        fn test_values_with_colons() {
            let test_file_path = test_file("format_colons");
            cleanup(&test_file_path);
            let mut env = LinoEnv::new(&test_file_path);
            env.set("URL", "https://example.com:8080");
            env.write().unwrap();

            let mut env2 = LinoEnv::new(&test_file_path);
            env2.read().unwrap();
            assert_eq!(
                env2.get("URL"),
                Some("https://example.com:8080".to_string())
            );
            cleanup(&test_file_path);
        }

        #[test]
        fn test_values_with_spaces() {
            let test_file_path = test_file("format_spaces");
            cleanup(&test_file_path);
            let mut env = LinoEnv::new(&test_file_path);
            env.set("MESSAGE", "Hello World");
            env.write().unwrap();

            let mut env2 = LinoEnv::new(&test_file_path);
            env2.read().unwrap();
            assert_eq!(env2.get("MESSAGE"), Some("Hello World".to_string()));
            cleanup(&test_file_path);
        }
    }

    mod multiline_quoted_value_tests {
        use super::*;

        #[test]
        fn test_multi_line_double_quoted_values() {
            let test_file_path = test_file("multiline_double_quoted");
            cleanup(&test_file_path);
            fs::write(
                &test_file_path,
                "HIVE_TELEGRAM_BOT_CONFIGURATION: \"\nTELEGRAM_BOT_TOKEN: 'xxx'\nTELEGRAM_ALLOWED_CHATS:\n  -1002975819706\nTELEGRAM_BOT_VERBOSE: true\n\"\nAFTER: value\n",
            )
            .unwrap();

            let mut env = LinoEnv::new(&test_file_path);
            env.read().unwrap();

            assert_eq!(
                env.get("HIVE_TELEGRAM_BOT_CONFIGURATION"),
                Some(
                    "\nTELEGRAM_BOT_TOKEN: 'xxx'\nTELEGRAM_ALLOWED_CHATS:\n  -1002975819706\nTELEGRAM_BOT_VERBOSE: true\n"
                        .to_string()
                )
            );
            assert_eq!(env.get("TELEGRAM_BOT_TOKEN"), None);
            assert_eq!(env.get("AFTER"), Some("value".to_string()));
            cleanup(&test_file_path);
        }

        #[test]
        fn test_multi_line_single_quoted_values() {
            let test_file_path = test_file("multiline_single_quoted");
            cleanup(&test_file_path);
            fs::write(&test_file_path, "SCRIPT: 'line1\nline2'\nAFTER: value\n").unwrap();

            let mut env = LinoEnv::new(&test_file_path);
            env.read().unwrap();

            assert_eq!(env.get("SCRIPT"), Some("line1\nline2".to_string()));
            assert_eq!(env.get("AFTER"), Some("value".to_string()));
            cleanup(&test_file_path);
        }
    }

    mod edge_case_tests {
        use super::*;

        #[test]
        fn test_nonexistent_file() {
            let test_file_path = test_file("nonexistent");
            cleanup(&test_file_path);
            let mut env = LinoEnv::new(&test_file_path);
            env.read().unwrap();

            assert_eq!(env.get("ANY_KEY"), None);
            assert!(env.keys().is_empty());
        }

        #[test]
        fn test_empty_values() {
            let test_file_path = test_file("empty_values");
            cleanup(&test_file_path);
            let mut env = LinoEnv::new(&test_file_path);
            env.set("EMPTY_KEY", "");
            env.write().unwrap();

            let mut env2 = LinoEnv::new(&test_file_path);
            env2.read().unwrap();
            assert_eq!(env2.get("EMPTY_KEY"), Some(String::new()));
            cleanup(&test_file_path);
        }
    }
}
