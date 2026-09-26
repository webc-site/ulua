use crate::records::lua_state::LuaState;

/// 读当前调用深度（`ci` 与 `base_ci` 的帧差）。B 档契约前移（参照
/// `abs_index`/`lua_mainthread` 先例）：原 `l` 须指向存活 `LuaState` 且
/// `(*l).ci`、`(*l).base_ci` 均落在其 `base_ci[0..size_ci]` 数组内、`ci ≥
/// base_ci`（差值非负）的契约改由 `&` 接收者+调用点既有 unsafe 块承载；被调体
/// 内残留的 `offset_from` 为裸指针差值固有 unsafe，收在既有 `unsafe` 块中，
/// 真 unsafe 操作零净增。纯指针差值，不解引用帧内容、不分配、不抛错。
/// cpp `ldebug.cpp:215`。
pub fn lua_stackdepth(l: &LuaState) -> i32 {
  // Safety: `l` 为存活 `LuaState` 引用，`ci`/`base_ci` 为其实例建档起维护的
  // 同数组内帧指针；此处仅取指针差值，不解引用任何帧内容。
  unsafe { l.ci.offset_from(l.base_ci) as i32 }
}
