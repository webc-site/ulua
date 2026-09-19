//! cpp Require.cpp 匿名命名空间的 `absolutePathPrefix`。
use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// 取首个 '/' 前的前缀, 镜像 cpp absolutePathPrefix
pub fn absolute_prefix(path: &str) -> String {
  let first_slash = path.find('/');
  LUAU_ASSERT!(first_slash.is_some());
  match first_slash {
    Some(idx) => path[..idx].to_string(),
    None => String::new(),
  }
}
