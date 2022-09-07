use crate::store::PluginStore;
use anyhow::{anyhow, Context, Result};
use semver::{Version, VersionReq};
use std::path::Path;

/// Checks whether the plugin supports the currently running version of Spin.
pub fn assert_supported_version(spin_version: &str, supported: &str) -> Result<()> {
    let supported = VersionReq::parse(supported).with_context(|| {
        format!(
            "could not parse manifest compatibility version {} as valid semver",
            supported,
        )
    })?;
    let version = Version::parse(spin_version)?;
    match supported.matches(&version) {
        true => Ok(()),
        false => Err(anyhow!(
            "plugin is compatible with Spin {} but running Spin {}",
            supported,
            spin_version
        )),
    }
}

/// Verifies that a plugin is compatible with the currently running version of Spin
/// by fetching it's manifest and assessing it's `spinCompatibility`.
pub fn check_plugin_spin_compatibility(
    plugin_name: &str,
    spin_version: &str,
    plugins_dir: &Path,
) -> Result<()> {
    let manifest = PluginStore::load_plugin_manifest(plugin_name, plugins_dir)?;
    assert_supported_version(spin_version, &manifest.spin_compatibility)
}

#[cfg(test)]
mod version_tests {
    use super::*;
    #[test]
    fn test_supported_version() {
        let test_case = ">=1.2.3, <1.8.0";
        let input_output = [
            ("1.3.0", true),
            ("1.2.3", true),
            ("1.8.0", false),
            ("1.9.0", false),
            ("1.2.0", false),
        ];
        input_output
            .into_iter()
            .for_each(|(i, o)| assert_eq!(assert_supported_version(i, test_case).is_err(), !o));
    }
}
