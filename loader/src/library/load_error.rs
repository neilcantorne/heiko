use std::fmt::Debug;

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

impl Debug for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unix { message } => f.write_str(message),
            Self::Windows { code } => f.debug_struct("Windows").field("code", code).finish(),
            Self::InvalidNameFormat => write!(f, "Invalid library name"),
            Self::ErrorMessageConversionError => write!(f, "Can't convert error message"),
        }
    }
}