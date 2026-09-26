use std::path::Path;

/// 路径绝对性的字节级判定核心（镜像 cpp `isAbsolutePath` 的 `string_view` 语义：
/// 直接比对原始字节，不做 Unicode 正规化）。
#[cfg(windows)]
fn bytes_is_absolute(bytes: &[u8]) -> bool {
  (bytes.len() >= 3
    && bytes[0].is_ascii_alphabetic()
    && bytes[1] == b':'
    && (bytes[2] == b'/' || bytes[2] == b'\\'))
    || (bytes.len() >= 1 && (bytes[0] == b'/' || bytes[0] == b'\\'))
}

#[cfg(not(windows))]
fn bytes_is_absolute(bytes: &[u8]) -> bool {
  matches!(bytes.first(), Some(b'/'))
}

pub fn is_absolute_path(path: &str) -> bool {
  bytes_is_absolute(path.as_bytes())
}

/// `Path` 形态的同语义判定：`as_encoded_bytes` 在 unix 下即原始字节、
/// windows 下保留 ASCII 分隔符，判定结果与 `&str` 版逐字节一致。
pub(crate) fn path_is_absolute(path: &Path) -> bool {
  bytes_is_absolute(path.as_os_str().as_encoded_bytes())
}
