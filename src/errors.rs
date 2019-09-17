use failure::Fail;

    /// `ENError` is the main error type. `EN` in `ENError` stands for the crate name `ElimuNasi`
#[derive(Fail, Debug)]
pub enum ENError {
        /// Toml Error type `toml::de::Error`
    #[fail(display = "{}", _0)]
    TomlError(#[cause] toml::de::Error),
        /// I/O Error type `std::io::Error`
    #[fail(display = "{}", _0)]
    IOError(#[cause] std::io::Error),
        /// I/O Error type `reql::errors::Error`
    #[fail(display = "{}", _0)]
    ReqlError(#[cause] reql::errors::Error),
        /// serde_json error propagation for `serde_json::error::Error`
    #[fail(display = "{}", _0)]
    SerdeJsonError(#[cause] serde_json::Error),
        /// SchemeGuardian error propagation for `schemeguardian::errors::SGError`
    #[fail(display = "{}", _0)]
    SGError(#[cause] schemeguardian::errors::SGError),
}

impl std::convert::From<toml::de::Error> for ENError {
    fn from(error: toml::de::Error) -> Self {
        ENError::TomlError(error)
    }
}

impl std::convert::From<std::io::Error> for ENError {
    fn from(error: std::io::Error) -> Self {
        ENError::IOError(error)
    }
}

impl std::convert::From<reql::errors::Error> for ENError {
    fn from(error: reql::errors::Error) -> Self {
        ENError::ReqlError(error)
    }
}

impl std::convert::From<serde_json::Error> for ENError {
    fn from(error: serde_json::Error) -> Self {
        ENError::SerdeJsonError(error)
    }
}

impl std::convert::From<schemeguardian::errors::SGError> for ENError {
    fn from(error: schemeguardian::errors::SGError) -> Self {
        ENError::SGError(error)
    }
}