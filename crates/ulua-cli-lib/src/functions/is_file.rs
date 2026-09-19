//! cpp `isFile`（CLI/src/FileUtils.cpp:341-351）的移植。
//!
//! 平台语义来自上游：Windows 走 `GetFileAttributesW`，判定「存在且非目录」；
//! POSIX 走 `lstat` + `S_IFREG`。两个分支分别实现，不共用 portable 判定。

/// Windows：镜像 cpp `GetFileAttributesW` 的「存在且非目录」语义 —— 属性拿不到
/// （路径不存在）为 false，否则只看 `FILE_ATTRIBUTE_DIRECTORY` 位。
///
/// 与同目录 `is_directory` 一样用 `symlink_metadata` 取 Win32 属性位，避免符号
/// 链接被 `metadata` 跟随后与上游 lstat 语义脱节。
#[cfg(windows)]
pub fn is_file(path: &str) -> bool {
  // FILE_ATTRIBUTE_DIRECTORY (win32 file attribute flags)
  const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;

  use std::{fs::symlink_metadata, os::windows::fs::MetadataExt};
  symlink_metadata(path)
    .map(|meta| (meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY) == 0)
    .unwrap_or(false)
}

/// 非 Windows：镜像 cpp 的 `lstat` + `S_IFREG`（symlink 本身只有指向常规文件才算
/// 存在，符号链接目录/断链均判 false）
#[cfg(not(windows))]
pub fn is_file(path: &str) -> bool {
  use std::fs::symlink_metadata;
  symlink_metadata(path)
    .map(|meta| meta.is_file())
    .unwrap_or(false)
}
