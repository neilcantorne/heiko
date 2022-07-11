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

use std::path::{ Path, PathBuf };

// Library search internal function
fn files_exists<const SHALLOW: bool, const LIBCOUNT: usize, T: AsRef<Path>>(path: T, names: &[&str; LIBCOUNT]) -> ([bool; LIBCOUNT], bool) {
    let mut result = [false; LIBCOUNT];
    let mut filled = 0usize;

    if let Ok(entries) = std::fs::read_dir(path) {

            for result_entries in entries {
            if let Ok(entry) = result_entries {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    for (index, name) in names.iter().enumerate() {
                        if entry.file_name() == *name {
                            result[index] = true;
                            filled += 1;
                        }
                    }
                }
                else {
                    if !SHALLOW {
                        let res = files_exists::<true, LIBCOUNT, PathBuf>(entry_path, names);

                        if res.1 {
                            return res;
                        }
                    }
                }
            }

            if filled >= LIBCOUNT {
                return (result, true);
            }
        }
    }

    (result, false)
}