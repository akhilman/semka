use crate::manifests::SiteManifest;
use crate::path::Path;
use seed::Url;

mod registry;
pub use registry::Registry;

#[derive(Debug)]
pub struct Context {
    pub url: Url,
    pub page_path: Path,
    pub base_path: Path,
    pub site_manifest: SiteManifest,
    pub registry: Registry,
}
