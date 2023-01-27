use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
// TODO: change this to #[serde(tag = "spin_manifest_version")]
// when ready to phase out backwards support of `spin_version`
#[serde(from = "AppSer")]
pub(crate) enum BuildAppInfoAnyVersion {
    #[serde(rename = "1")]
    V1(BuildAppInfoV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct BuildAppInfoV1 {
    #[serde(rename = "component")]
    pub components: Vec<RawComponentManifest>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AppSer {
    OldSkool(AppOldSkool),
    NewSkool(AppNewSkool),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "spin_version")]
enum AppOldSkool {
    #[serde(rename = "1")]
    V1(BuildAppInfoV1),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "spin_manifest_version")]
enum AppNewSkool {
    #[serde(rename = "1")]
    V1(BuildAppInfoV1),
}

impl From<AppSer> for BuildAppInfoAnyVersion {
    fn from(value: AppSer) -> Self {
        match value {
            AppSer::OldSkool(AppOldSkool::V1(m)) => Self::V1(m),
            AppSer::NewSkool(AppNewSkool::V1(m)) => Self::V1(m),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct RawComponentManifest {
    pub id: String,
    pub build: Option<RawBuildConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub(crate) struct RawBuildConfig {
    pub command: String,
    pub workdir: Option<PathBuf>,
}
