use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::functions::{has_suffix::has_suffix, is_absolute_path::is_absolute_path};

/// cpp Require.cpp 的 kInitSuffixes（目录 init 模块后缀）。
pub(crate) const K_INIT_SUFFIXES: &[&str] = &["/init.luau", "/init.lua"];

/// cpp Require.cpp 的 kSuffixes（常规模块后缀）。
pub(crate) const K_SUFFIXES: &[&str] = &[".luau", ".lua"];

/// 合并视图：init 后缀先于常规后缀，与 cpp kInitSuffixes/kSuffixes 两段循环等价。
pub(crate) const K_MODULE_SUFFIXES: &[&str] = &["/init.luau", "/init.lua", ".luau", ".lua"];

pub fn get_module_path(file_path: &str) -> String {
  let file_path_normalized = file_path.replace('\\', "/");

  let mut path_view: &str = &file_path_normalized;

  if is_absolute_path(path_view) {
    let first_slash = path_view.find('/');
    LUAU_ASSERT!(first_slash.is_some());
    if let Some(idx) = first_slash {
      path_view = &path_view[idx..];
    }
  }

  for &suffix in K_MODULE_SUFFIXES {
    if has_suffix(path_view, suffix) {
      path_view = &path_view[..path_view.len() - suffix.len()];
      return path_view.to_string();
    }
  }

  path_view.to_string()
}
