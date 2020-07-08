use crate::path;
use failure::Fail;

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not parse path: {}", _0)]
pub struct ParsePathError(&'static str);

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not fetch {}, {}", path, error)]
pub struct FetchError {
    pub path: path::AbsPath,
    pub error: String,
}

impl FetchError {
    pub fn new(path: path::AbsPath, error: String) -> Self {
        Self { path, error }
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
