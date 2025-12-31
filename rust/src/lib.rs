//! LinoEnv - A Rust library to read and write .lenv files.
//!
//! `.lenv` files use `: ` instead of `=` for key-value separation.
//! Example: `GITHUB_TOKEN: gh_....`

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Package version (matches Cargo.toml version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// LinoEnv - A struct to read and write .lenv files.
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
/// let path = "/tmp/test_lino_env_example.lenv";
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
    data: HashMap<String, Vec<String>>,
}

impl LinoEnv {
    /// Create a new LinoEnv instance.
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
    /// Stores all instances of each key (duplicates are allowed).
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

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Parse line with `: ` separator
            if let Some(separator_index) = line.find(": ") {
                let key = line[..separator_index].trim().to_string();
                let value = line[separator_index + 2..].to_string(); // Don't trim value to preserve spaces

                self.data.entry(key).or_default().push(value);
            }
        }

        Ok(self)
    }

    /// Get the last instance of a reference (key).
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to look up
    ///
    /// # Returns
    ///
    /// The last value associated with the key, or None if not found.
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
        self.data
            .get(reference)
            .and_then(|values| values.last().cloned())
    }

    /// Get all instances of a reference (key).
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to look up
    ///
    /// # Returns
    ///
    /// All values associated with the key, or an empty vector if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.add("KEY", "value1");
    /// env.add("KEY", "value2");
    /// assert_eq!(env.get_all("KEY"), vec!["value1", "value2"]);
    /// ```
    #[must_use]
    pub fn get_all(&self, reference: &str) -> Vec<String> {
        self.data.get(reference).cloned().unwrap_or_default()
    }

    /// Set all instances of a reference to a new value.
    ///
    /// Replaces all existing instances with a single new value.
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
    /// env.add("KEY", "old1");
    /// env.add("KEY", "old2");
    /// env.set("KEY", "new_value");
    /// assert_eq!(env.get_all("KEY"), vec!["new_value"]);
    /// ```
    pub fn set(&mut self, reference: &str, value: &str) -> &mut Self {
        self.data
            .insert(reference.to_string(), vec![value.to_string()]);
        self
    }

    /// Add a new instance of a reference (allows duplicates).
    ///
    /// # Arguments
    ///
    /// * `reference` - The key to add
    /// * `value` - The value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.add("KEY", "value1");
    /// env.add("KEY", "value2");
    /// assert_eq!(env.get_all("KEY"), vec!["value1", "value2"]);
    /// ```
    pub fn add(&mut self, reference: &str, value: &str) -> &mut Self {
        self.data
            .entry(reference.to_string())
            .or_default()
            .push(value.to_string());
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
    /// let path = "/tmp/test_lino_env_write.lenv";
    /// let mut env = LinoEnv::new(path);
    /// env.set("KEY", "value");
    /// env.write().unwrap();
    ///
    /// // Clean up
    /// fs::remove_file(path).ok();
    /// ```
    pub fn write(&self) -> io::Result<&Self> {
        let mut file = fs::File::create(&self.file_path)?;

        for (key, values) in &self.data {
            for value in values {
                writeln!(file, "{key}: {value}")?;
            }
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
    /// `true` if the key exists and has at least one value.
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
        self.data
            .get(reference)
            .is_some_and(|values| !values.is_empty())
    }

    /// Delete all instances of a reference.
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

    /// Get all entries as a HashMap (with last instance of each key).
    ///
    /// # Returns
    ///
    /// A HashMap with each key mapped to its last value.
    ///
    /// # Examples
    ///
    /// ```
    /// use lino_env::LinoEnv;
    /// let mut env = LinoEnv::new(".lenv");
    /// env.add("KEY1", "value1a");
    /// env.add("KEY1", "value1b");
    /// env.set("KEY2", "value2");
    /// let obj = env.to_hash_map();
    /// assert_eq!(obj.get("KEY1"), Some(&"value1b".to_string()));
    /// ```
    #[must_use]
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (key, values) in &self.data {
            if let Some(last_value) = values.last() {
                result.insert(key.clone(), last_value.clone());
            }
        }
        result
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
/// let path = "/tmp/test_write_lino_env.lenv";
/// let mut data = HashMap::new();
/// data.insert("KEY".to_string(), "value".to_string());
/// write_lino_env(path, &data).unwrap();
///
/// // Clean up
/// fs::remove_file(path).ok();
/// ```
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
            .join(format!("lino_env_test_{}.lenv", name))
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
        fn test_get_last_instance() {
            let test_file = test_file("get_last_instance");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.add("API_KEY", "value1");
            env.add("API_KEY", "value2");
            env.add("API_KEY", "value3");

            assert_eq!(env.get("API_KEY"), Some("value3".to_string()));
            cleanup(&test_file);
        }

        #[test]
        fn test_get_nonexistent() {
            let test_file = test_file("get_nonexistent");
            let env = LinoEnv::new(&test_file);
            assert_eq!(env.get("NON_EXISTENT"), None);
        }
    }

    mod get_all_tests {
        use super::*;

        #[test]
        fn test_get_all_instances() {
            let test_file = test_file("get_all_instances");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.add("API_KEY", "value1");
            env.add("API_KEY", "value2");
            env.add("API_KEY", "value3");

            assert_eq!(env.get_all("API_KEY"), vec!["value1", "value2", "value3"]);
            cleanup(&test_file);
        }

        #[test]
        fn test_get_all_nonexistent() {
            let test_file = test_file("get_all_nonexistent");
            let env = LinoEnv::new(&test_file);
            assert!(env.get_all("NON_EXISTENT").is_empty());
        }
    }

    mod set_tests {
        use super::*;

        #[test]
        fn test_set_replaces_all() {
            let test_file = test_file("set_replaces_all");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.add("API_KEY", "value1");
            env.add("API_KEY", "value2");
            env.set("API_KEY", "new_value");

            assert_eq!(env.get("API_KEY"), Some("new_value".to_string()));
            assert_eq!(env.get_all("API_KEY"), vec!["new_value"]);
            cleanup(&test_file);
        }
    }

    mod add_tests {
        use super::*;

        #[test]
        fn test_add_duplicates() {
            let test_file = test_file("add_duplicates");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.add("KEY", "value1");
            env.add("KEY", "value2");
            env.add("KEY", "value3");

            assert_eq!(env.get_all("KEY"), vec!["value1", "value2", "value3"]);
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
        fn test_delete_all_instances() {
            let test_file = test_file("delete_all_instances");
            cleanup(&test_file);
            let mut env = LinoEnv::new(&test_file);
            env.add("KEY", "value1");
            env.add("KEY", "value2");
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
            env.add("KEY1", "value1a");
            env.add("KEY1", "value1b");
            env.set("KEY2", "value2");

            let obj = env.to_hash_map();
            assert_eq!(obj.get("KEY1"), Some(&"value1b".to_string()));
            assert_eq!(obj.get("KEY2"), Some(&"value2".to_string()));
            cleanup(&test_file);
        }
    }

    mod persistence_tests {
        use super::*;

        #[test]
        fn test_persist_duplicates() {
            let test_file = test_file("persist_duplicates");
            cleanup(&test_file);
            let mut env1 = LinoEnv::new(&test_file);
            env1.add("KEY", "value1");
            env1.add("KEY", "value2");
            env1.add("KEY", "value3");
            env1.write().unwrap();

            let mut env2 = LinoEnv::new(&test_file);
            env2.read().unwrap();

            assert_eq!(env2.get_all("KEY"), vec!["value1", "value2", "value3"]);
            assert_eq!(env2.get("KEY"), Some("value3".to_string()));
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
