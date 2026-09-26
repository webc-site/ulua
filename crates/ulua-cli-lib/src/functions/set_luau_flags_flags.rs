use ulua_common::records::f_value::FValue;

use crate::functions::set_luau_flag::set_luau_flag;

pub(crate) fn set_luau_flags_bool(state: bool) {
  FValue::<bool>::set_all_unless(state, |name| !name.starts_with("Luau"));
}

/// cpp `setLuauFlags`: `true`/`True` 与 `false`/`False` 两组大小写变体的布尔解析
/// （非全大小写无关，与 cpp 逐字面量比较等价）；`None` 表示非法值。四枚雷同字面量
/// 在此单点表述，键值分支与裸开关分支共用。
fn parse_bool(value: &str) -> Option<bool> {
  match value {
    "true" | "True" => Some(true),
    "false" | "False" => Some(false),
    _ => None,
  }
}

/// 镜像 cpp `setLuauFlags(const char* list)`: 逗号分隔的 flag 列表。
/// 每项形如 `name`、`name=true/false` (大小写变体均可), 或裸 `true`/`false`
/// 表示全局开关。
pub fn set_luau_flags(list: &str) {
  for part in list.split(',') {
    if part.is_empty() {
      continue;
    }
    if let Some((key, value)) = part.split_once('=') {
      match parse_bool(value) {
        Some(state) => set_luau_flag(key, state),
        None => eprintln!("Warning: unrecognized value '{value}' for flag '{key}'."),
      }
    } else if let Some(state) = parse_bool(part) {
      set_luau_flags_bool(state);
    } else {
      set_luau_flag(part, true);
    }
  }
}
