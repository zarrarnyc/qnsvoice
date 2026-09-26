use crate::CoreResult;
use std::path::{Path, PathBuf};
use tracing::info;

/// Plugin loader for discovering and loading plugins from a directory
pub struct PluginLoader {
    plugins_dir: PathBuf,
}

impl PluginLoader {
    /// Create a new plugin loader for the given directory
    pub fn new<P: AsRef<Path>>(plugins_dir: P) -> Self {
        Self {
            plugins_dir: plugins_dir.as_ref().to_path_buf(),
        }
    }

    /// Load a plugin by name
    ///
    /// Reads the plugin's manifest.toml and registers it
    pub fn load(&self, plugin_name: &str) -> CoreResult<()> {
        let plugin_dir = self.plugins_dir.join(plugin_name);
        let manifest_path = plugin_dir.join("manifest.toml");

        if !manifest_path.exists() {
            return Err(crate::CoreError::Plugin(format!(
                "Plugin '{}' manifest not found at {}",
                plugin_name,
                manifest_path.display()
            )));
        }

        info!("Loading plugin from: {}", manifest_path.display());

        // Read and parse manifest
        let content = std::fs::read_to_string(&manifest_path).map_err(|e| {
            crate::CoreError::Plugin(format!(
                "Failed to read manifest for '{}': {}",
                plugin_name, e
            ))
        })?;

        let _manifest: PluginManifest = toml::from_str(&content).map_err(|e| {
            crate::CoreError::Plugin(format!(
                "Failed to parse manifest for '{}': {}",
                plugin_name, e
            ))
        })?;

        info!("Plugin '{}' loaded successfully", plugin_name);
        Ok(())
    }
}

/// Plugin manifest structure (from manifest.toml)
#[derive(Debug, serde::Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub config: Option<toml::Value>,
}
