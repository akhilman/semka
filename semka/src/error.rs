use failure::Fail;

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not parse path: {}", _0)]
pub struct ParsePathError(&'static str);

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not fetch {}, {}", url, error)]
pub struct FetchError {
    pub url: String,
    pub error: String,
}

impl FetchError {
    pub fn new(url: impl AsRef<str>, error: seed::fetch::FetchError) -> Self {
        Self {
            url: url.as_ref().to_string(),
            error: format!("{:?}", error),
        }
    }
}

#[derive(Fail, Debug, Clone)]
pub enum SemkaError {
    #[fail(display = "Error: {}", _0)]
    ParsePathError(ParsePathError),
    #[fail(display = "Error: {}", _0)]
    FetchError(FetchError),
}

impl From<FetchError> for SemkaError {
    fn from(err: FetchError) -> Self {
        SemkaError::FetchError(err)
    }
}
