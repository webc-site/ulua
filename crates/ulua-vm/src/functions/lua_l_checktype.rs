use crate::{
  functions::{lua_type::lua_type, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须是正在执行的 C 函数帧且 `narg` 为合法栈索引（`lua_type` 据此取槽）；标签不符时经
/// `tag_error` 抛 Lua 错误、不返回。索引失真会按错误槽判型，把非错误值当参数放行。cpp laux.cpp:164。
pub(crate) unsafe fn lua_l_checktype(l: *mut LuaState, narg: i32, t: i32) {
  // Safety: 契约保证 `l` 为存活调用帧且 narg 栈槽可读；标签不符时经 lua_type 名称对其抛错、不返回
  unsafe {
    if lua_type(l, narg) != t {
      tag_error(l, narg, t);
    }
  }
}
