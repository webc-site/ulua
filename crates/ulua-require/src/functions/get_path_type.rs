use crate::enums::path_type::PathType;

/// 相对当前目录的前缀（对应 cpp `path.substr(0, 2) == "./"`）。
const PREFIX_CURRENT: &[u8] = b"./";
/// 相对父目录的前缀（对应 cpp `path.substr(0, 3) == "../"`）。
const PREFIX_PARENT: &[u8] = b"../";
/// 别名路径前缀字节（对应 cpp `path[0] == '@'`）。
pub(crate) const ALIAS_PREFIX: u8 = b'@';

/// 按前缀判定路径类型；纯字节比较，非法 UTF-8 不影响判定。
pub fn get_path_type(path: &[u8]) -> PathType {
  if path.starts_with(PREFIX_CURRENT) {
    return PathType::RelativeToCurrent;
  }
  if path.starts_with(PREFIX_PARENT) {
    return PathType::RelativeToParent;
  }
  if path.first() == Some(&ALIAS_PREFIX) {
    return PathType::Aliased;
  }

  PathType::Unsupported
}
