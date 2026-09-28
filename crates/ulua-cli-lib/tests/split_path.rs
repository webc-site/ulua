//! `split_path` 对齐 cpp `Common.cpp` 的 `splitPath`：按 `/` 与 `\` 两种分隔符
//! 切分，并保留空段（前导/尾随分隔符都会产出 `""`），因为调用方用返回的
//! 段数判断根路径与相对路径。

use ulua_cli_lib::functions::split_path::split_path;

#[test]
fn splits_on_both_separators() {
  assert_eq!(split_path("a/b\\c"), ["a", "b", "c"]);
  assert_eq!(split_path("/abs"), ["", "abs"]);
  assert_eq!(split_path("trailing/"), ["trailing", ""]);
  assert_eq!(split_path(""), [""]);
}
