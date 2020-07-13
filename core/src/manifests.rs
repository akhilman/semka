use crate::path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteManifest {
    pub root_document: path::DocPath,
}

impl Default for SiteManifest {
    fn default() -> Self {
        Self {
            root_document: "emptySite".parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocManifest {
    pub widget: String,
}
