#[cfg(windows)]
pub fn is_directory(path: &str) -> bool {
  use std::{fs::symlink_metadata, os::windows::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.file_attributes() & 0x10) != 0)
    .unwrap_or(false)
}

#[cfg(not(windows))]
pub fn is_directory(path: &str) -> bool {
  use std::{fs::symlink_metadata, os::unix::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.mode() & 0xf000) == 0x4000)
    .unwrap_or(false)
}
