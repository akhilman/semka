use failure::Fail;

pub type Result<T> = std::result::Result<T, SemkaError>;

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not parse path: {}", _0)]
pub struct ParsePathError(&'static str);

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not fetch {}, {}", url, error)]
pub struct FetchError {
    // TODO replace with failure::Context
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
#[fail(display = "Error in widget {} {}", widget, err)]
pub struct WidgetError {
    widget: String,
    err: &'static str,
}

impl WidgetError {
    pub fn new(widget: impl AsRef<str>, err: &'static str) -> Self {
        Self {
            widget: widget.as_ref().to_string(),
            err,
        }
    }
}

// TODO: replace with failure::Error
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
