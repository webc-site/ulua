#[cfg(windows)]
pub fn is_directory(path: &str) -> bool {
  // FILE_ATTRIBUTE_DIRECTORY (win32 file attribute flags)
  const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;

  use std::{fs::symlink_metadata, os::windows::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY) != 0)
    .unwrap_or(false)
}

#[cfg(not(windows))]
pub fn is_directory(path: &str) -> bool {
  // POSIX lstat: S_IFMT 类型掩码 + S_IFDIR 目录类型位
  const S_IFMT: u32 = 0o170000;
  const S_IFDIR: u32 = 0o040000;

  use std::{fs::symlink_metadata, os::unix::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.mode() & S_IFMT) == S_IFDIR)
    .unwrap_or(false)
}
