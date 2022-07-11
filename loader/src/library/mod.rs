#[cfg(target_family = "unix")]
mod internal {
    mod lib_unix;
    pub(crate) use lib_unix::Library;
}

#[cfg(target_os = "windows")]
mod internal {
    mod lib_windows;
    pub(crate) use lib_windows::Library;
}

mod load_error;

pub(crate) use internal::Library;
pub(crate) use load_error::LoadError;