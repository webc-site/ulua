use core::{
  mem::{size_of, zeroed},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, buffer_swapbe::SwapBe, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_pushnumber::lua_pushnumber,
  },
  macros::isoutofbounds::isoutofbounds,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn buffer_readinteger<T>(l: *mut lua_State) -> i32
where
  T: SwapBe + Into<f64>,
{
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);

    if isoutofbounds(offset, len, size_of::<T>()) {
      buffer_oob_error(l);
    }

    let mut val: T = zeroed();
    copy_nonoverlapping(
      (buf as *const u8).add(offset as usize),
      &mut val as *mut T as *mut u8,
      size_of::<T>(),
    );

    if LUAU_BIG_ENDIAN {
      val = val.swap_be();
    }

    lua_pushnumber(l, val.into());
    1
  }
}
