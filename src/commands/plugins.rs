use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use spin_plugins::install::PluginInstaller;
use std::path::PathBuf;

const SPIN_PLUGINS_REPO: &str =
    "https://raw.githubusercontent.com/karthik2804/spin-plugins/main/plugins/";

/// Install/uninstall plugins
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

/// Install plugins from remote source
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
        PluginInstaller::new(&self.name, SPIN_PLUGINS_REPO, get_spin_plugins_directory()?)?
            .install()
            .await?;
        Ok(())
    }
}

/// Remove the specified plugin
#[derive(Parser, Debug)]
pub struct Uninstall {
    /// Name of Spin plugin.
    pub name: String,
    // TODO: think about how to handle breaking changes
    // #[structopt(long = "update")]
    // pub update: bool,
}

impl Uninstall {
    pub async fn run(self) -> Result<()> {
        println!("The plugin {:?} will be removed", self.name);
        Ok(())
    }
}

/// Gets the path to where Spin plugin are (to be) installed
pub fn get_spin_plugins_directory() -> anyhow::Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|p| p.join(".spin")))
        .ok_or_else(|| anyhow!("Unable to get local data directory or home directory"))?;
    let plugins_dir = data_dir.join("spin").join("plugins");
    Ok(plugins_dir)
}
