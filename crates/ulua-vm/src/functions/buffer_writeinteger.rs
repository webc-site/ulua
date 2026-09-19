use core::{
  ffi::{c_char, c_int},
  mem::{size_of, transmute_copy},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, buffer_swapbe::buffer_swapbe,
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  macros::isoutofbounds::isoutofbounds,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn buffer_writeinteger<T>(l: *mut lua_State) -> c_int
where
  T: Copy,
{
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len) as *mut c_char;
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkunsigned(l, 3);

    if isoutofbounds(offset, len, size_of::<T>()) {
      buffer_oob_error(l);
    }

    let mut val: T = transmute_copy::<u32, T>(&value);

    if LUAU_BIG_ENDIAN {
      val = buffer_swapbe(val);
    }

    copy_nonoverlapping(
      &val as *const T as *const u8,
      (buf as *mut u8).add(offset as usize),
      size_of::<T>(),
    );
    0
  }
}
