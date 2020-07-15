use crate::constants;
use crate::error;
use crate::manifests;
use crate::path;
use futures::future::{Future, TryFutureExt};
use std::borrow::Borrow;

pub fn fetch_json<U, T>(url: U) -> impl Future<Output = Result<T, error::FetchError>>
where
    U: ToString,
    T: serde::de::DeserializeOwned + 'static,
{
    let url = url.to_string();
    let url_clone = url.clone();
    async {
        let manifest = seed::browser::fetch::fetch(url)
            .await?
            .check_status()?
            .json::<T>()
            .await?;
        Ok(manifest)
    }
    .map_err(|err| error::FetchError::from_seed(url_clone, err))
}

pub async fn fetch_site_manifest() -> Result<manifests::SiteManifest, error::FetchError> {
    fetch_json(constants::SITE_MANIFEST_FILE).await
}

pub async fn fetch_doc_manifest(
    doc_path: impl Borrow<path::Path>,
) -> Result<manifests::DocManifest, error::FetchError> {
    let doc_name = doc_path
        .borrow()
        .iter()
        .nth(0)
        .ok_or(error::FetchError::RequestError(
            doc_path.borrow().to_string(),
            "Document path is empty".to_string(),
        ))?;
    let doc_manifest_path = path::Path::new()
        .add(constants::DOC_DIR)
        .add(doc_name)
        .add(constants::DOC_MANIFEST_FILE);
    let manifest: manifests::DocManifest = fetch_json(doc_manifest_path).await?;
    Ok(manifest)
}
