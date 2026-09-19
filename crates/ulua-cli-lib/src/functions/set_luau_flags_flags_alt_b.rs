use crate::functions::{set_luau_flag::set_luau_flag, set_luau_flags_flags::set_luau_flags_bool};

/// 镜像 cpp `setLuauFlags(const char* list)`: 逗号分隔的 flag 列表。
/// 每项形如 `name`、`name=true/false` (大小写变体均可), 或裸 `true`/`false`
/// 表示全局开关。
pub fn set_luau_flags(list: &str) {
  for part in list.split(',') {
    if part.is_empty() {
      continue;
    }
    if let Some((key, value)) = part.split_once('=') {
      if value == "true" || value == "True" {
        set_luau_flag(key, true);
      } else if value == "false" || value == "False" {
        set_luau_flag(key, false);
      } else {
        eprintln!("Warning: unrecognized value '{value}' for flag '{key}'.");
      }
    } else if part == "true" || part == "True" {
      set_luau_flags_bool(true);
    } else if part == "false" || part == "False" {
      set_luau_flags_bool(false);
    } else {
      set_luau_flag(part, true);
    }
  }
}
