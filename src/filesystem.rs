use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

use normpath::PathExt;

use crate::dir_entry;

pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let path = path.strip_prefix(".").unwrap_or(path);
    env::current_dir().map(|path_buf| path_buf.join(path))
}

pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    path_absolute_form(path)
}

pub fn is_existing_directory(path: &Path) -> bool {
    // Note: we do not use `.exists()` here, as `.` always exists, even if
    // the CWD has been deleted.
    path.is_dir() && (path.file_name().is_some() || path.normalize().is_ok())
}

pub fn is_empty(entry: &dir_entry::DirEntry) -> bool {
    if let Some(file_type) = entry.file_type() {
        if file_type.is_dir() {
            if let Ok(mut entries) = fs::read_dir(entry.path()) {
                entries.next().is_none()
            } else {
                false
            }
        } else if file_type.is_file() {
            entry.metadata().map(|m| m.len() == 0).unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(any(unix, target_os = "redox"))]
pub fn is_block_device(ft: fs::FileType) -> bool {
    ft.is_block_device()
}

#[cfg(any(unix, target_os = "redox"))]
pub fn is_char_device(ft: fs::FileType) -> bool {
    ft.is_char_device()
}

#[cfg(any(unix, target_os = "redox"))]
pub fn is_socket(ft: fs::FileType) -> bool {
    ft.is_socket()
}

#[cfg(any(unix, target_os = "redox"))]
pub fn is_pipe(ft: fs::FileType) -> bool {
    ft.is_fifo()
}

#[cfg(any(unix, target_os = "redox"))]
pub fn osstr_to_bytes(input: &OsStr) -> Cow<'_, [u8]> {
    use std::os::unix::ffi::OsStrExt;
    Cow::Borrowed(input.as_bytes())
}

/// Remove the `./` prefix from a path.
pub fn strip_current_dir(path: &Path) -> &Path {
    path.strip_prefix(".").unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::strip_current_dir;
    use std::path::Path;

    #[test]
    fn strip_current_dir_basic() {
        assert_eq!(strip_current_dir(Path::new("./foo")), Path::new("foo"));
        assert_eq!(strip_current_dir(Path::new("foo")), Path::new("foo"));
        assert_eq!(
            strip_current_dir(Path::new("./foo/bar/baz")),
            Path::new("foo/bar/baz")
        );
        assert_eq!(
            strip_current_dir(Path::new("foo/bar/baz")),
            Path::new("foo/bar/baz")
        );
    }
}
