use crate::path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteManifest {
    #[serde(default)]
    pub index_page: path::PagePath,
    #[serde(default)]
    pub master_page: path::PagePath,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocManifest {
    pub widget: String,
}
