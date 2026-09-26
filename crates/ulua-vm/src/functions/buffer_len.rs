use crate::{
  functions::{lua_l_checkbuffer::lua_l_checkbuffer, lua_pushnumber::lua_pushnumber},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_len(l: *mut LuaState) -> i32 {
  let mut len: usize = 0;
  // Safety: 契约保证 `l` 存活且实参 1 为已检查 buffer userdata，其 len 字段即数据界
  unsafe {
    lua_l_checkbuffer(l, 1, &mut len);
    lua_pushnumber(l, len as f64);
  }
  1
}

lua_lib_fn!(pub(crate) fn buffer_len, buffer_len_arm);
