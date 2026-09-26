use core::slice::from_raw_parts;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_pushlstring::lua_pushlstring,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_argcheck::luaL_argcheck},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe extern "C-unwind" fn buffer_readstring(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向本次 buffer 库调用的存活 LuaState，buffer 数据界由已校验的 userdata 长度字段给出，越界访问统一走报错路径
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len).cast::<u8>();
    let offset = lua_l_checkinteger(l, 2);
    let size = lua_l_checkinteger(l, 3);

    luaL_argcheck!(l, size >= 0, 3, "size");

    if isoutofbounds(offset, len, size as usize) {
      buffer_oob_error(l);
    }

    // isoutofbounds 已保证 [offset, offset+size) 落在 buffer 数据界内
    let region = from_raw_parts(buf.add(offset as usize), size as usize);
    lua_pushlstring(l, region.as_ptr().cast(), region.len());

    1
  }
}
