use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const CONFIG_FILE: &str = "tartarus_params.toml";

/// The parameters of the hashing algorithm, saved to disk and loaded at startup.
/// The `pepper` is not stored here : it is a server-side secret read from the `TARTARUS_PEPPER` environment variable.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CliParams {
    pub memory: u32,
    pub iterations: u32,
}

impl Default for CliParams {
    fn default() -> Self {
        Self {
            memory: 12,
            iterations: 3,
        }
    }
}

impl CliParams {
    pub fn config_path() -> PathBuf {
        PathBuf::from(CONFIG_FILE)
    }

    pub fn load() -> Self {
        Self::load_from(&Self::config_path()).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_to(&Self::config_path())
    }

    pub fn load_from(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        toml::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn save_to(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())
    }

    /// Returns a copy with the given fields overridden. `None` keeps the current value.
    pub fn apply(&self, memory: Option<u32>, iterations: Option<u32>) -> Self {
        let mut updated = *self;
        if let Some(memory) = memory {
            updated.memory = memory;
        }
        if let Some(iterations) = iterations {
            updated.iterations = iterations;
        }
        updated
    }
}

#[cfg(test)]
mod tests {
    use super::CliParams;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("tartarus_params_test_{}_{}", name, std::process::id()))
    }

    #[test]
    fn defaults_are_hardcoded() {
        let params = CliParams::default();
        assert_eq!(params, CliParams { memory: 12, iterations: 3 });
    }

    #[test]
    fn save_then_load_round_trip() {
        let path = temp_path("round_trip");
        let params = CliParams { memory: 64, iterations: 2 };

        params.save_to(&path).unwrap();
        let loaded = CliParams::load_from(&path).unwrap();

        assert_eq!(loaded, params);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn missing_file_returns_error() {
        let path = temp_path("missing");
        std::fs::remove_file(&path).ok();

        assert!(CliParams::load_from(&path).is_err());
    }

    #[test]
    fn corrupt_file_returns_error() {
        let path = temp_path("corrupt");
        std::fs::write(&path, "this is not valid toml =").unwrap();

        assert!(CliParams::load_from(&path).is_err());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn serializes_to_expected_toml() {
        let params = CliParams { memory: 64, iterations: 2 };
        let content = toml::to_string(&params).unwrap();
        assert_eq!(content, "memory = 64\niterations = 2\n");
    }

    #[test]
    fn deserializes_from_toml() {
        let params: CliParams = toml::from_str("memory = 128\niterations = 1\n").unwrap();
        assert_eq!(params, CliParams { memory: 128, iterations: 1 });
    }

    #[test]
    fn save_to_directory_path_fails() {
        let path = temp_path("directory");
        std::fs::create_dir_all(&path).unwrap();

        assert!(CliParams::default().save_to(&path).is_err());
        std::fs::remove_dir_all(&path).ok();
    }

    #[test]
    fn apply_updates_only_given_fields() {
        let base = CliParams::default();

        assert_eq!(base.apply(Some(64), None), CliParams { memory: 64, iterations: 3 });
        assert_eq!(base.apply(None, Some(2)), CliParams { memory: 12, iterations: 2 });
        assert_eq!(base.apply(Some(64), Some(2)), CliParams { memory: 64, iterations: 2 });
        assert_eq!(base.apply(None, None), base);
    }
}
