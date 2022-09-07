mod git;
pub mod install;
mod plugin_manifest;
mod prompt;
pub mod store;
pub mod uninstall;
pub mod version_check;

/// List of Spin internal subcommands
pub(crate) const SPIN_INTERNAL_COMMANDS: [&str; 9] = [
    "templates",
    "up",
    "new",
    "bindle",
    "deploy",
    "build",
    "plugin",
    "trigger",
    "external",
];
