use core::slice::from_raw_parts_mut;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::isoutofbounds::isoutofbounds,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe extern "C-unwind" fn buffer_fill(l: *mut LuaState) -> i32 {
  // Safety: 契约保证填充区间经 argcheck 落在 buffer 数据界内，memset 仅触达该界
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len).cast::<u8>();
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkunsigned(l, 3);
    // C++ evaluates `int(len) - offset` as the default eagerly (signed overflow
    // is UB upstream for offset = INT_MIN); wrapping_sub reproduces the two's-
    // complement value C++ relies on, which the `size < 0` / isoutofbounds checks
    // below then reject. (Upstream UBSan: lbuflib.cpp:278.)
    let size = lua_l_optinteger(l, 4, (len as i32).wrapping_sub(offset));

    if size < 0 {
      buffer_oob_error(l);
    }

    if isoutofbounds(offset, len, size as usize) {
      buffer_oob_error(l);
    }

    from_raw_parts_mut(buf.add(offset as usize), size as usize).fill((value & 0xff) as u8);

    0
  }
}
