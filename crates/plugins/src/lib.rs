mod git;
// TODO: just export PluginInstaller
pub mod install;
mod plugin_manifest;
mod prompt;
pub mod uninstall;
pub mod version_check;

/// Directory where the manifests of installed plugins are stored.
const PLUGIN_MANIFESTS_DIRECTORY_NAME: &str = "manifests";

fn get_manifest_file_name(plugin_name: &str) -> String {
    format!("{}.json", plugin_name)
}
