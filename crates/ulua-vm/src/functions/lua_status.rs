use crate::records::lua_state::LuaState;

/// 读该线程的状态码（`LuaStatus` 枚举判别值，`i32` 形状透传给 C ABI）。
/// B 档契约前移（参照 `abs_index`/`lua_mainthread` 先例）：原 `*mut LuaState`
/// 存活契约改由 `&` 接收者的引用有效性规则在调用点承载；本体仅读 `status`
/// 一个字段、零 unsafe 操作，签名转 safe fn。cpp `lapi.cpp:1294`。
pub fn lua_status(l: &LuaState) -> i32 {
  l.status as i32
}
