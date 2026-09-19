use core::{ffi::c_int, mem::size_of, ptr::copy_nonoverlapping};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, buffer_swapbe::SwapBe,
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::isoutofbounds::isoutofbounds,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn buffer_readlong(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);

    if isoutofbounds(offset, len, size_of::<u64>()) {
      buffer_oob_error(l);
    }

    let mut val: u64 = 0;
    copy_nonoverlapping(
      (buf as *const u8).add(offset as usize),
      &mut val as *mut u64 as *mut u8,
      size_of::<u64>(),
    );

    if LUAU_BIG_ENDIAN {
      val = val.swap_be();
    }

    lua_pushinteger_64(l, val as i64);
    1
  }
}
