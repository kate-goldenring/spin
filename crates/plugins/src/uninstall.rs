use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::store::PluginStore;

/// Settings for uninstalling a plugin.
pub struct PluginUninstaller {
    /// Name of plugin to be uninstalled.
    plugin_name: String,
    /// Path to the directory where plugins are installed.
    plugins_dir: PathBuf,
}

impl PluginUninstaller {
    pub fn new(plugin_name: &str, plugins_dir: PathBuf) -> Self {
        Self {
            plugin_name: plugin_name.to_owned(),
            plugins_dir,
        }
    }

    /// Uninstalls a plugin with a given name, removing it and it's manifest
    /// from the local plugins directory.
    pub fn run(&self) -> Result<()> {
        // Check if plugin is installed
        let manifest_file =
            PluginStore::installed_manifest_path(&self.plugins_dir, &self.plugin_name);
        let plugin_exists = manifest_file.exists();
        match plugin_exists {
            // Remove the manifest and the plugin installation directory
            true => {
                fs::remove_file(manifest_file)?;
                fs::remove_dir_all(self.plugins_dir.join(&self.plugin_name))?;
                println!("Uninstalled plugin {} successfully", &self.plugin_name);
            }
            false => {
                println!(
                    "The following plugin  \"{}\" does not exist, therefore cannot be uninstalled",
                    self.plugin_name
                );
            }
        }
        Ok(())
    }
}
