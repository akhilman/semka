use crate::manifests::SiteManifest;
use crate::path::{AbsPath, PagePath};
use seed::Url;

mod registry;
pub use registry::Registry;

#[derive(Debug)]
pub struct Context {
    pub url: Url,
    pub page_path: PagePath,
    pub base_path: AbsPath,
    pub site_manifest: SiteManifest,
    pub registry: Registry,
}
