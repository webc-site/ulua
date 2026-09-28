//! Faithful port of `static std::string getFilePath(const char* name)`
//! (`CLI/src/Repl.cpp:570`).

use alloc::string::String;

use ulua_cli_lib::functions::is_file::is_file;

/// 依次尝试 `name`、`name.luau`、`name.lua`，返回首个存在的路径。
/// cpp 以空串表示「都不存在」，本端口用 [`Option`] 表达。
pub(crate) fn get_file_path(name: &str) -> Option<String> {
  if is_file(name) {
    return Some(name.into());
  }
  for ext in [".luau", ".lua"] {
    let candidate = alloc::format!("{name}{ext}");
    if is_file(&candidate) {
      return Some(candidate);
    }
  }
  None
}
