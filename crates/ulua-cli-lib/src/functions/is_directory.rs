use std::path::Path;

/// Windows：镜像 cpp `GetFileAttributesW` 的「存在且为目录」语义 —— 属性拿不到
/// （路径不存在）为 false，否则只看 `FILE_ATTRIBUTE_DIRECTORY` 位。
///
/// 与 `is_file_path` 一样用 `symlink_metadata` 取 Win32 属性位，避免符号链接被
/// `metadata` 跟随后与上游 lstat 语义脱节。
#[cfg(windows)]
pub(crate) fn is_directory_path(path: &Path) -> bool {
  // FILE_ATTRIBUTE_DIRECTORY (win32 file attribute flags)
  const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;

  use std::{fs::symlink_metadata, os::windows::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY) != 0)
    .unwrap_or(false)
}

/// 非 Windows：镜像 cpp 的 `lstat` + `S_IFDIR`（只看路径本身，不跟随符号链接）。
/// `symlink_metadata().is_dir()` 即 lstat 后取 `S_IFMT == S_IFDIR`，无需手写掩码。
#[cfg(not(windows))]
pub(crate) fn is_directory_path(path: &Path) -> bool {
  use std::fs::symlink_metadata;
  symlink_metadata(path)
    .map(|meta| meta.is_dir())
    .unwrap_or(false)
}

/// `&str` 便捷入口（公开 API 保持不变），内部统一收敛到 `is_directory_path`。
pub fn is_directory(path: &str) -> bool {
  is_directory_path(Path::new(path))
}
