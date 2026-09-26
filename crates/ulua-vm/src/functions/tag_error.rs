use crate::{
  functions::{lua_l_typeerror_l::lua_l_typeerror_l, lua_typename::lua_typename_str},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn tag_error(l: *mut LuaState, narg: i32, tag: i32) -> ! {
  let tname = lua_typename_str(tag).unwrap_or("unknown");
  // Safety: 直接转发 `lua_l_typeerror_l`；`l` 存活与 narg 可读由本函数契约传递，调用后抛错不返回
  unsafe { lua_l_typeerror_l(l, narg, tname) }
}
