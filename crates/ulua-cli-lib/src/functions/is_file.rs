use std::fs::symlink_metadata;

pub fn is_file(path: &str) -> bool {
  // 用 symlink_metadata + is_file() 镜像 POSIX lstat/S_IFREG, 各平台正确
  symlink_metadata(path)
    .map(|meta| meta.is_file())
    .unwrap_or(false)
}
