use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tonumberx::lua_tonumberx, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证 `l` 为正在执行的 C 函数帧、`narg` 为其中合法栈索引；取数经安全门面
/// `lua_tonumberx`，非数值时 `tag_error` 经 `l` 抛 Lua 错误且不返回。cpp laux.cpp:196。
pub(crate) fn lua_l_checknumber(l: &mut LuaState, narg: i32) -> f64 {
  match lua_tonumberx(l, narg) {
    Some(d) => d,
    // SAFETY: 契约保证 `l` 为存活调用帧（`&mut` 接收者承载存活）且 narg 栈槽可读；
    // 非数值路径 `tag_error` 经 `l` 抛错不返回
    None => unsafe { tag_error(l, narg, LuaType::Number as i32) },
  }
}

// lualib.h name
