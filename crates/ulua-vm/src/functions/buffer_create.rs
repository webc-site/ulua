use crate::{
  functions::lua_newbuffer::lua_newbuffer, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe fn buffer_create(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活调用帧且实参 1 经 checkinteger 为非负数，新 buffer 按该大小分配
  unsafe {
    let size = (*l).check_integer(1);

    (*l).arg_check(size >= 0, 1, "size");

    lua_newbuffer(l, size as usize);
    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_create, buffer_create_arm);
