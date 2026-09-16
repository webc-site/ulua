use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_module_path::{K_INIT_SUFFIXES, K_SUFFIXES},
    is_directory::is_directory,
    is_file::is_file,
  },
  records::resolved_real_path::ResolvedRealPath,
};

/// 尝试候选后缀: 第二个匹配 → Ambiguous, 首个匹配记录 suffix
fn try_suffix<'a>(
  module_path: &str,
  suffix: &mut &'a str,
  found: &mut bool,
  potential_suffix: &'a str,
) -> Option<NavigationStatus> {
  let mut path_with_suffix = module_path.to_string();
  path_with_suffix.push_str(potential_suffix);
  if !is_file(&path_with_suffix) {
    return None;
  }
  if *found {
    return Some(NavigationStatus::Ambiguous);
  }
  *suffix = potential_suffix;
  *found = true;
  None
}

pub fn get_real_path(module_path: String) -> ResolvedRealPath {
  let mut found = false;
  let mut suffix = "";

  let last_slash = module_path.rfind('/');
  LUAU_ASSERT!(last_slash.is_some());

  let last_component = match last_slash {
    Some(idx) => &module_path[idx + 1..],
    None => "",
  };

  if last_component != "init" {
    for &potential_suffix in K_SUFFIXES {
      if let Some(status) = try_suffix(&module_path, &mut suffix, &mut found, potential_suffix) {
        return ResolvedRealPath::new(status, String::new());
      }
    }
  }

  if is_directory(&module_path) {
    if found {
      return ResolvedRealPath::new(NavigationStatus::Ambiguous, String::new());
    }

    for &potential_suffix in K_INIT_SUFFIXES {
      if let Some(status) = try_suffix(&module_path, &mut suffix, &mut found, potential_suffix) {
        return ResolvedRealPath::new(status, String::new());
      }
    }

    found = true;
  }

  if !found {
    return ResolvedRealPath::new(NavigationStatus::NotFound, String::new());
  }

  let mut result_path = module_path;
  result_path.push_str(suffix);
  ResolvedRealPath::new(NavigationStatus::Success, result_path)
}
