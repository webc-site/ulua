use crate::enums::path_type::PathType;

pub fn get_path_type(path: &str) -> PathType {
  if path.starts_with("./") {
    return PathType::RelativeToCurrent;
  }
  if path.starts_with("../") {
    return PathType::RelativeToParent;
  }
  if path.starts_with('@') {
    return PathType::Aliased;
  }

  PathType::Unsupported
}
