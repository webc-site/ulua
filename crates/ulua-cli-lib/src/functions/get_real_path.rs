use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{is_directory::is_directory, is_file::is_file},
  records::resolved_real_path::ResolvedRealPath,
};

const K_INIT_SUFFIXES: &[&str] = &["/init.luau", "/init.lua"];
const K_SUFFIXES: &[&str] = &[".luau", ".lua"];

pub fn get_real_path(module_path: String) -> ResolvedRealPath {
  let mut found = false;
  let mut suffix = "";

  let last_slash = module_path.rfind('/');
  LUAU_ASSERT!(last_slash.is_some());

  let last_component = if let Some(idx) = last_slash {
    &module_path[idx + 1..]
  } else {
    ""
  };

  if last_component != "init" {
    for &potential_suffix in K_SUFFIXES {
      let mut path_with_suffix = module_path.clone();
      path_with_suffix.push_str(potential_suffix);
      if is_file(&path_with_suffix) {
        if found {
          return ResolvedRealPath::new(NavigationStatus::Ambiguous, String::new());
        }

        suffix = potential_suffix;
        found = true;
      }
    }
  }

  if is_directory(&module_path) {
    if found {
      return ResolvedRealPath::new(NavigationStatus::Ambiguous, String::new());
    }

    for &potential_suffix in K_INIT_SUFFIXES {
      let mut path_with_suffix = module_path.clone();
      path_with_suffix.push_str(potential_suffix);
      if is_file(&path_with_suffix) {
        if found {
          return ResolvedRealPath::new(NavigationStatus::Ambiguous, String::new());
        }

        suffix = potential_suffix;
        found = true;
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
