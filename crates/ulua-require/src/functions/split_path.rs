/// 路径分隔符字节（对应 cpp `splitPath` 的 `'/'`）；Lua 路径是字节串，非 UTF-8。
pub(crate) const PATH_SEPARATOR: u8 = b'/';
/// cpp `navigate` 里被 `std::replace` 归一成分隔符的反斜杠字节。
pub(crate) const PATH_SEPARATOR_ALT: u8 = b'\\';

/// 对应 cpp `splitPath`：按首个分隔符切分，无分隔符时第二段为空。
pub fn split_path(path: &[u8]) -> (&[u8], &[u8]) {
  match memchr::memchr(PATH_SEPARATOR, path) {
    Some(pos) => (&path[..pos], &path[pos + 1..]),
    None => (path, &[]),
  }
}
