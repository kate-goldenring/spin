use super::plugin::{Os, Plugin};
use super::prompt::Prompter;
use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use std::{
    fs::{self, File},
    io::{copy, Cursor},
    path::{Path, PathBuf},
};
use tar::Archive;
use tempfile::{tempdir, TempDir};
use url::Url;

/// Name of the subdirectory that contains the installed plugin JSON manifests
const PLUGIN_MANIFESTS_DIRECTORY_NAME: &str = "manifests";

pub struct PluginInstaller {
    name: String,
    url: Url,
    plugins_dir: PathBuf,
}

impl PluginInstaller {
    pub fn new(name: &str, url: &str, plugins_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            url: Url::parse(url)?.join(&get_manifest_file_name(name))?,
            plugins_dir,
        })
    }
    pub async fn install(&self) -> Result<()> {
        // TODO: Potentially handle errors to give useful error messages
        log::info!(
            "Trying to get tar file for plugin manifest from {}",
            self.url
        );
        let plugin = reqwest::get(self.url.to_owned())
            .await?
            .json::<Plugin>()
            .await?;
        let os: Os = if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else if cfg!(target_os = "macos") {
            Os::MacOs
        } else {
            return Err(anyhow!("This plugin is not supported on this OS"));
        };
        // TODO: Add logic for architecture as well
        let plugin_package = plugin
            .packages
            .iter()
            .find(|p| p.os == os)
            .ok_or_else(|| anyhow!("This plugin does not support this OS"))?;
        let target_url = plugin_package.url.to_owned();
        // TODO: Ask for User confirmation
        if !Prompter::new(
            &plugin.name,
            &plugin.license,
            self.url.to_owned(),
            &target_url,
        )?
        .run()?
        {
            // User has requested to not install package, returning early
            println!("Plugin {} will not be installed", plugin.name);
            return Ok(());
        }
        // TODO: Handle licensing of plugins

        let temp_dir = tempdir()?;
        let plugin_file_name = self.download_plugin(&temp_dir, &target_url).await?;
        self.verify_checksum(&plugin_file_name, &plugin_package.sha256)?;

        self.untar_plugin(&plugin_file_name)?;
        // Save manifest to installed plugins directory
        self.add_to_manifest_dir(&plugin)?;
        Ok(())
    }

    fn untar_plugin(&self, plugin_file_name: &PathBuf) -> Result<()> {
        // Get handle to file
        let tar_gz = File::open(&plugin_file_name)?;
        // Unzip file
        let tar = GzDecoder::new(tar_gz);
        // Get plugin from tarball
        let mut archive = Archive::new(tar);
        // TODO: this is unix only. Look into whether permissions are preserved
        archive.set_preserve_permissions(true);
        // Create subdirectory in plugins directory for this plugin
        let plugin_sub_dir = self.plugins_dir.join(&self.name);
        fs::create_dir_all(&plugin_sub_dir)?;
        archive.unpack(&plugin_sub_dir)?;
        Ok(())
    }

    async fn download_plugin(&self, temp_dir: &TempDir, target_url: &str) -> Result<PathBuf> {
        log::info!(
            "Trying to get tar file for plugin {} from {}",
            self.name,
            target_url
        );
        let plugin_bin = reqwest::get(target_url).await?;
        let mut content = Cursor::new(plugin_bin.bytes().await?);
        let dir = temp_dir.path();
        let mut plugin_file = dir.join(&self.name);
        plugin_file.set_extension("tar.gz");
        let mut temp_file = File::create(&plugin_file)?;
        copy(&mut content, &mut temp_file)?;
        Ok(plugin_file)
    }

    // Validate checksum of downloaded content with checksum from Index
    fn verify_checksum(&self, plugin_file: &PathBuf, checksum: &str) -> Result<()> {
        let binary_sha256 = file_digest_string(plugin_file).expect("failed to get sha for parcel");
        let verification_sha256 = checksum;
        if binary_sha256 == verification_sha256 {
            println!("Package verified successfully");
            Ok(())
        } else {
            Err(anyhow!("Could not validate Checksum"))
        }
    }

    fn add_to_manifest_dir(&self, plugin: &Plugin) -> Result<()> {
        let manifests_dir = self.plugins_dir.join(PLUGIN_MANIFESTS_DIRECTORY_NAME);
        fs::create_dir_all(&manifests_dir)?;
        serde_json::to_writer(
            &File::create(manifests_dir.join(get_manifest_file_name(&plugin.name)))?,
            plugin,
        )?;
        Ok(())
    }
}

fn get_manifest_file_name(plugin_name: &str) -> String {
    format!("{}.json", plugin_name)
}

fn file_digest_string(path: impl AsRef<Path>) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut file = std::fs::File::open(&path)?;
    let mut sha = Sha256::new();
    std::io::copy(&mut file, &mut sha)?;
    let digest_value = sha.finalize();
    let digest_string = format!("{:x}", digest_value);
    Ok(digest_string)
}
