mod git;
mod plugin_manifest;
// TODO: just export PluginInstaller
pub mod install;
mod prompt;
pub mod uninstall;

/// Directory where the manifests of installed plugins are stored.
const PLUGIN_MANIFESTS_DIRECTORY_NAME: &str = "manifests";

fn get_manifest_file_name(plugin_name: &str) -> String {
    format!("{}.json", plugin_name)
}
