#[cfg(target_os = "linux")]
mod internal {
    mod lib_linux;
    pub(crate) use lib_linux::Library;
}

#[cfg(target_os = "windows")]
mod internal {
    mod lib_windows;
    pub(crate) use lib_windows::Library;
}

#[cfg(target_os = "macos")]
mod internal {
    mod lib_macos;
    pub(crate) use lib_macos::Library;
}

mod load_error;

pub(crate) use internal::Library;
pub(crate) use load_error::LoadError;