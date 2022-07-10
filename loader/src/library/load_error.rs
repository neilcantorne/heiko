pub(crate) enum LoadError {
    Unix { message: String },
    Windows { code: u32 },
    InvalidNameFormat,
    ErrorMessageConversionError
}