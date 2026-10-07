use crate::{
  functions::{lua_type::lua_type, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证 `l` 为正在执行的 C 函数帧且 `narg` 为合法栈索引（`lua_type` 据此取槽）；
/// 判型经安全门面 `lua_type`，标签不符时 `tag_error` 经 `l` 抛 Lua 错误、不返回。
/// 索引失真会按错误槽判型，把非错误值当参数放行。cpp laux.cpp:164。
pub(crate) fn lua_l_checktype(l: &mut LuaState, narg: i32, t: i32) {
  if lua_type(l, narg) != t {
    // SAFETY: 契约保证 `l` 为存活调用帧（`&mut` 接收者承载存活）且 narg 栈槽可读；
    // 标签不符时 `tag_error` 经 `l` 抛错不返回
    unsafe { tag_error(l, narg, t) };
  }
}
