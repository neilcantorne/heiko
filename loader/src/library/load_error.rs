pub(crate) enum LoadError {
    Unix { message: String },
    Windows { code: u32 },
    InvalidNameFormat,
    ErrorMessageConversionError
}

impl From<std::ffi::NulError> for LoadError {
    fn from(_: std::ffi::NulError) -> Self {
        Self::InvalidNameFormat
    }
}

impl From<std::str::Utf8Error> for LoadError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::ErrorMessageConversionError
    }
}