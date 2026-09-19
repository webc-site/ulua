/// 末段扩展名（含 `.`）：最后一个分隔符/点若为点则返回其后缀，否则空串。
///
/// 直接借用入参切片，零分配（cpp 版返回 `std::string`）。
pub fn get_extension(path: &str) -> &str {
  match path.rfind(['.', '\\', '/']) {
    Some(dot_index) if path.as_bytes()[dot_index] == b'.' => &path[dot_index..],
    _ => "",
  }
}
