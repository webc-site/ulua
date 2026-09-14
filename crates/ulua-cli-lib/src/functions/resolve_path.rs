use alloc::string::String;

use crate::functions::{
  get_parent_path::get_parent_path,
  join_paths_file_utils_alt_b::join_paths_string_view_string_view, normalize_path::normalize_path,
};

pub fn resolve_path(path: &str, base_file_path: &str) -> Option<String> {
  let base_file_path_parent = get_parent_path(base_file_path)?;
  let joined = join_paths_string_view_string_view(&base_file_path_parent, path);
  Some(normalize_path(&joined))
}
