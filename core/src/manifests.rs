use crate::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteManifest {
    #[serde(default)]
    pub index_page: Path,
    #[serde(default)]
    pub master_page: Path,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocManifest {
    pub widget: String,
}
