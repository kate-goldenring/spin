use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use comfy_table::Table;

// use spin_plugins::{
//     InstallOptions, InstallationResults, InstalledPluginWarning, ListResults, ProgressReporter,
//     SkippedReason, PluginManager, PluginSource,
// };
use crate::opts::PLUGIN_NAME_OPT;

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// Install plugin from the Spin plugin repository.
    ///
    /// The binary or .wasm file of the plugin is copied to the local Spin plugins directory
    /// TODO: consider the ability to install multiple plugins
    Install(Install),

    /// Remove a plugin from your installation.
    Uninstall(Uninstall),

    // TODO: consider Search command
    
    // TODO: consider List command
}

impl PluginCommands {
    pub async fn run(self) -> Result<()> {
        match self {
            PluginCommands::Install(cmd) => cmd.run().await,
            PluginCommands::Uninstall(cmd) => cmd.run().await,
        }
    }
}

/// Install plugins from a Git repository or local directory.
#[derive(Parser, Debug)]
pub struct Install {
    /// Name of Spin plugin.
    pub name: String,
    // If present, updates existing plugins instead of skipping.
    // TODO: think about how to handle breaking changes
    // #[structopt(long = "update")]
    // pub update: bool,
}

impl Install {
    pub async fn run(self) -> Result<()> {
        println!("The name of the plugin being installed {:?}", self.name);
        Ok(())
    }
}

/// Install plugins from a Git repository or local directory.
#[derive(Parser, Debug)]
pub struct Uninstall {
    /// Name of Spin plugin.
    #[clap(
        name = PLUGIN_NAME_OPT,
        long = "plugin-name",
    )]
    pub name: String,
    // If present, updates existing plugins instead of skipping.
    // TODO: think about how to handle breaking changes
    // #[structopt(long = "update")]
    // pub update: bool,
}

impl Uninstall {
    pub async fn run(self) -> Result<()> {
        println!("The name of the plugin being uninstalled {:?}", self.name);
        Ok(())
    }
}