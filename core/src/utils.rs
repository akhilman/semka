use crate::error;
use futures::future::{Future, TryFutureExt};

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
    .map_err(|err| error::FetchError::new(url_clone, err))
}
