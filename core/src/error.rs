use failure::Fail;

pub type Result<T> = std::result::Result<T, SemkaError>;

#[derive(Fail, Debug, Clone)]
#[fail(display = "Can not parse path: {}", _0)]
pub struct ParsePathError(&'static str);

#[derive(Fail, Debug, Clone)]
pub enum FetchError {
    #[fail(display = "Can not fetch \"{}\". Serde error {}", _0, _1)]
    SerdeError(String, String),
    #[fail(display = "Can not fetch \"{}\". DOM exception {}", _0, _1)]
    DomException(String, String),
    #[fail(display = "Can not fetch \"{}\". Promise error {}", _0, _1)]
    PromiseError(String, String),
    #[fail(display = "Can not fetch \"{}\". Network error {}", _0, _1)]
    NetworkError(String, String),
    /// Request construction failed.
    #[fail(display = "Can not fetch \"{}\". Request error {}", _0, _1)]
    RequestError(String, String),
    #[fail(display = "Can not fetch \"{}\". Not found {} {}", url, code, text)]
    NotFound {
        url: String,
        code: u16,
        text: String,
    },
    #[fail(display = "Can not fetch \"{}\". Forbidden {} {}", url, code, text)]
    Forbidden {
        url: String,
        code: u16,
        text: String,
    },
    #[fail(display = "Can not fetch \"{}\". Client error {} {}", url, code, text)]
    ClientError {
        url: String,
        code: u16,
        text: String,
    },
    #[fail(display = "Can not fetch \"{}\". Server error {} {}", url, code, text)]
    ServerError {
        url: String,
        code: u16,
        text: String,
    },
    #[fail(display = "Can not fetch \"{}\". Unknown Error {} {}", url, code, text)]
    UnknownError {
        url: String,
        code: u16,
        text: String,
    },
}

impl FetchError {
    pub fn from_seed(url: impl AsRef<str>, error: seed::fetch::FetchError) -> Self {
        use seed::fetch::FetchError as E;
        let url = url.as_ref().to_string();
        match error {
            E::SerdeError(e) => FetchError::SerdeError(url, format!("{}", e)),
            E::DomException(e) => FetchError::DomException(url, format!("{:?}", e)),
            E::PromiseError(v) => FetchError::PromiseError(url, format!("{:?}", v)),
            E::NetworkError(v) => FetchError::NetworkError(url, format!("{:?}", v)),
            E::RequestError(v) => FetchError::RequestError(url, format!("{:?}", v)),
            E::StatusError(status) => {
                use seed::browser::fetch::StatusCategory;
                if status.code == 404 {
                    FetchError::NotFound {
                        url,
                        code: status.code,
                        text: status.text,
                    }
                } else if status.code == 403 {
                    FetchError::Forbidden {
                        url,
                        code: status.code,
                        text: status.text,
                    }
                } else if status.category == StatusCategory::ClientError {
                    FetchError::ClientError {
                        url,
                        code: status.code,
                        text: status.text,
                    }
                } else if status.category == StatusCategory::ServerError {
                    FetchError::ServerError {
                        url,
                        code: status.code,
                        text: status.text,
                    }
                } else {
                    FetchError::UnknownError {
                        url,
                        code: status.code,
                        text: status.text,
                    }
                }
            }
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
