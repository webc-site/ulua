//! CLI 路径小工具的输入→输出契约，逐条对照 `./cpp` oracle：`joinPaths`
//! 两个重载（`cpp/CLI/src/FileUtils.cpp:227-233` 模板版与 `:388-395` 的
//! `string_view` 版）、`normalizePath`（`:92-149`）。
//!
//! `join_paths` 与 `normalize_path` 的改写（两份拼接实现收敛为一个带
//! `rhs_sep_matters` 参数的入口、`normalize_path` 第 3 步改用切片 `join`）由
//! 本文件把守；`atoi` 前缀语义收敛到 `option_parsing.rs`，不再重复。

use ulua_cli_lib::functions::{join_paths_file_utils::join_paths, normalize_path::normalize_path};

/// `rhs_sep_matters = false`：cpp:388-395 `string_view` 版，只看 lhs 尾部
#[test]
fn join_paths_string_view_only_checks_lhs() {
  assert_eq!(join_paths("a", "b", false), "a/b");
  assert_eq!(join_paths("a/", "b", false), "a/b");
  assert_eq!(join_paths("", "b", false), "b");
  assert_eq!(join_paths("a", "/b", false), "a//b");
}

/// `rhs_sep_matters = true`：cpp:227-233 模板版，lhs 尾部与 rhs 首部都不是
/// 分隔符才补 '/'
#[test]
fn join_paths_basic_string_checks_both_sides() {
  assert_eq!(join_paths("a", "b", true), "a/b");
  assert_eq!(join_paths("a/", "b", true), "a/b");
  assert_eq!(join_paths("a", "/b", true), "a/b");
  assert_eq!(join_paths("", "b", true), "b");
  // 空 rhs 的 '\0' 通过 `*rhs != '/' && *rhs != '\\'` 判定，照样补 '/'
  // （cpp 模板版语义，钉死防止实现回退成 `!rhs.is_empty()` 分支）
  assert_eq!(join_paths("a", "", true), "a/");
}

#[test]
fn normalize_path_matches_cpp() {
  assert_eq!(normalize_path("a/b/../c"), "./a/c");
  assert_eq!(normalize_path("/etc/passwd"), "/etc/passwd");
  assert_eq!(normalize_path("./a/./b"), "./a/b");
  assert_eq!(normalize_path("a/.."), "./");
  // 相对路径逃逸到父级：不加 "./" 前缀，尾部 ".." 补 '/'
  assert_eq!(normalize_path("../x"), "../x");
  assert_eq!(normalize_path("a/../.."), "../");
  // 绝对路径吞掉越界的 ".."（cpp：isAbsolute 时空 normalizedComponents 不入栈）
  assert_eq!(normalize_path("/a/../../b"), "/b");
}
