use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_newbuffer::lua_newbuffer},
  macros::lua_l_argcheck::luaL_argcheck,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe extern "C-unwind" fn buffer_create(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧且实参 1 经 checkinteger 为非负数，新 buffer 按该大小分配
  unsafe {
    let size = lua_l_checkinteger(l, 1);

    luaL_argcheck!(l, size >= 0, 1, "size");

    lua_newbuffer(l, size as usize);
    1
  }
}
