use anyhow::{Context, Result};
use semver::Version;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::plugin_manifest::PluginManifest;

/// Directory where the manifests of installed plugins are stored.
pub const PLUGIN_MANIFESTS_DIRECTORY_NAME: &str = "manifests";
// Name of directory that contains the cloned centralized Spin plugins
// repository
const PLUGINS_REPO_LOCAL_DIRECTORY: &str = ".spin-plugins";
// Name of directory containing the installed manifests
const PLUGINS_REPO_MANIFESTS_DIRECTORY: &str = "manifests";

/// Houses utilities for getting the path to Spin plugin directories.
pub struct PluginStore;

impl PluginStore {
    /// Given a plugin name, returns the expected file name for the installed manifest
    pub fn manifest_file_name(plugin_name: &str) -> String {
        format!("{}.json", plugin_name)
    }

    // Given a name and option version, outputs expected file name for the plugin.
    fn manifest_file_name_version(plugin_name: &str, version: &Option<semver::Version>) -> String {
        match version {
            Some(v) => format!("{}@{}.json", plugin_name, v),
            None => PluginStore::manifest_file_name(plugin_name),
        }
    }

    /// Get expected path to the manifest of a plugin with a given name
    /// and version within the spin-plugins repository
    // TODO: consider adding plugins_dir to struct as field
    pub fn spin_plugins_repo_manifest_path(
        plugins_dir: &Path,
        plugin_name: &str,
        plugin_version: &Option<Version>,
    ) -> PathBuf {
        plugins_dir
            .join(PLUGINS_REPO_LOCAL_DIRECTORY)
            .join(PLUGINS_REPO_MANIFESTS_DIRECTORY)
            .join(plugin_name)
            .join(PluginStore::manifest_file_name_version(
                plugin_name,
                plugin_version,
            ))
    }

    /// Get the path to the subdirectory of an installed plugin.
    pub fn plugin_subdirectory_path(plugins_dir: &Path, plugin_name: &str) -> PathBuf {
        plugins_dir.join(plugin_name)
    }

    /// Get the path to the manifests directory which contains the plugin manifests
    /// of all installed Spin plugins.
    pub fn installed_manifests_directory(plugins_dir: &Path) -> PathBuf {
        plugins_dir.join(PLUGIN_MANIFESTS_DIRECTORY_NAME)
    }

    /// Checks that the `spin-plugins` repository has been cloned locally for looking up plugins.
    pub fn plugin_manifests_repo_exists(plugins_dir: &Path) -> bool {
        PluginStore::plugin_manifests_repo_path(plugins_dir)
            .join(".git")
            .exists()
    }

    pub fn plugin_manifests_repo_path(plugins_dir: &Path) -> PathBuf {
        plugins_dir.join(PLUGINS_REPO_LOCAL_DIRECTORY)
    }

    pub fn installed_manifest_path(plugins_dir: &Path, plugin_name: &str) -> PathBuf {
        plugins_dir
            .join(PLUGIN_MANIFESTS_DIRECTORY_NAME)
            .join(PluginStore::manifest_file_name(plugin_name))
    }

    /// Returns the PluginManifest for an installed plugin with a given name.
    /// Looks up and parses the JSON plugin manifest file into object form.
    pub fn load_plugin_manifest(plugin_name: &str, plugins_dir: &Path) -> Result<PluginManifest> {
        let manifest_path = PluginStore::installed_manifest_path(plugins_dir, plugin_name);
        log::info!("Reading plugin manifest from {}", manifest_path.display());
        let manifest_file = File::open(manifest_path.clone()).with_context(|| {
            format!(
                "The plugin manifest does not exist at {}",
                manifest_path.display()
            )
        })?;
        let manifest = serde_json::from_reader(manifest_file)?;
        Ok(manifest)
    }
}
