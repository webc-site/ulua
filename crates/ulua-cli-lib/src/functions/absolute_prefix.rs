//! cpp/CLI/src/VfsNavigator.cpp 内联计算的 `absolutePathPrefix`
//! （`resetToPath`/`resetToStdIn`/`resetToAlias` 三处 `substr(0, firstSlash)`）。
use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// 取首个 '/' 前的前缀, 镜像 cpp absolutePathPrefix
pub(crate) fn absolute_prefix(path: &str) -> String {
  let first_slash = path.find('/');
  LUAU_ASSERT!(first_slash.is_some());
  match first_slash {
    Some(idx) => path[..idx].to_string(),
    None => String::new(),
  }
}
