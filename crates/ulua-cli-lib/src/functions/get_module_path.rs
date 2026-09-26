use std::path::{Path, PathBuf};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::functions::is_absolute_path::is_absolute_path;

/// cpp/CLI/src/VfsNavigator.cpp 的 kInitSuffixes（目录 init 模块后缀）。
pub(crate) const K_INIT_SUFFIXES: &[&str] = &["/init.luau", "/init.lua"];

/// cpp/CLI/src/VfsNavigator.cpp 的 kSuffixes（常规模块后缀）。
pub(crate) const K_SUFFIXES: &[&str] = &[".luau", ".lua"];

/// 模块后缀全集：init 后缀先于常规后缀，与 cpp kInitSuffixes/kSuffixes 两段
/// 循环的先后次序一致。
const MODULE_SUFFIXES: [&[&str]; 2] = [K_INIT_SUFFIXES, K_SUFFIXES];

/// 剥掉 `path` 尾部的模块后缀；无后缀命中时原样返回（借用入参，零拷贝）。
pub(crate) fn strip_module_suffix(path: &str) -> &str {
  MODULE_SUFFIXES
    .into_iter()
    .flatten()
    .find_map(|&suffix| path.strip_suffix(suffix))
    .unwrap_or(path)
}

/// 文件路径 → 模块路径（cpp `getModulePath`，按值收 `std::string`，此处以
/// `&Path` 借用、`PathBuf` 归还）。`to_string_lossy` 对源自 UTF-8 的输入
/// （本 crate 的全部上游数据）零拷贝恒等，仅在平台异常编码时退化。
pub(crate) fn get_module_path(file_path: &Path) -> PathBuf {
  let file_path_normalized = file_path.to_string_lossy().replace('\\', "/");

  let mut path_view: &str = &file_path_normalized;

  if is_absolute_path(path_view) {
    let first_slash = path_view.find('/');
    LUAU_ASSERT!(first_slash.is_some());
    if let Some(idx) = first_slash {
      path_view = &path_view[idx..];
    }
  }

  PathBuf::from(strip_module_suffix(path_view))
}
