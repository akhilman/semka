use crate::constants;
use crate::error;
use crate::manifests;
use crate::path;
use bytes::Bytes;
use futures::future::{Future, TryFutureExt};

pub fn fetch_bytes<U>(url: U) -> impl Future<Output = Result<Bytes, error::FetchError>>
where
    U: ToString,
{
    let url = url.to_string();
    let url_clone = url.clone();
    async {
        let bytes = seed::browser::fetch::fetch(url)
            .await?
            .check_status()?
            .bytes()
            .await
            .map(Bytes::from)?;
        Ok(bytes)
    }
    .map_err(|err| error::FetchError::from_seed(url_clone, err))
}

pub fn fetch_json<U, T>(url: U) -> impl Future<Output = Result<T, error::FetchError>>
where
    U: ToString,
    T: serde::de::DeserializeOwned + 'static,
{
    let url = url.to_string();
    let url_clone = url.clone();
    async {
        let value = seed::browser::fetch::fetch(url)
            .await?
            .check_status()?
            .json::<T>()
            .await?;
        Ok(value)
    }
    .map_err(|err| error::FetchError::from_seed(url_clone, err))
}

pub fn fetch_text<U>(url: U) -> impl Future<Output = Result<String, error::FetchError>>
where
    U: ToString,
{
    let url = url.to_string();
    let url_clone = url.clone();
    async {
        let text = seed::browser::fetch::fetch(url)
            .await?
            .check_status()?
            .text()
            .await?;
        Ok(text)
    }
    .map_err(|err| error::FetchError::from_seed(url_clone, err))
}

pub async fn fetch_site_manifest() -> Result<manifests::SiteManifest, error::FetchError> {
    fetch_json(constants::SITE_MANIFEST_FILE).await
}

pub async fn fetch_doc_manifest(
    doc_name: impl AsRef<str>,
) -> Result<manifests::DocManifest, error::FetchError> {
    let doc_name = doc_name.as_ref();
    if doc_name.is_empty() {
        return Err(error::FetchError::RequestError(
            doc_name.to_string(),
            "Document name is empty".to_string(),
        ));
    }
    let doc_manifest_path = path::Path::new()
        .add(constants::DOC_DIR)
        .add(doc_name)
        .add(constants::DOC_MANIFEST_FILE);
    let manifest: manifests::DocManifest = fetch_json(doc_manifest_path).await?;
    Ok(manifest)
}

pub fn show_spinner<Ms>() -> seed::prelude::Node<Ms> {
    use seed::{prelude::*, *};
    div!["Loading..."]
}
