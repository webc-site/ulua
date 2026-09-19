use core::{
  ffi::{c_char, c_int},
  mem::{size_of, zeroed},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, buffer_swapbe::SwapBe,
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_pushnumber::lua_pushnumber,
  },
  macros::isoutofbounds::isoutofbounds,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn buffer_readfp<T, StorageType>(l: *mut lua_State) -> c_int
where
  T: Copy + BufferReadableFloat,
  StorageType: SwapBe,
{
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len) as *mut c_char;
    let offset = lua_l_checkinteger(l, 2);

    if isoutofbounds(offset, len, size_of::<T>()) {
      buffer_oob_error(l);
    }

    let mut val: T = zeroed();

    if LUAU_BIG_ENDIAN {
      static_assert_size::<T, StorageType>();
      let mut tmp: StorageType = zeroed();
      copy_nonoverlapping(
        buf.add(offset as usize),
        &mut tmp as *mut StorageType as *mut c_char,
        size_of::<StorageType>(),
      );
      tmp = tmp.swap_be();
      copy_nonoverlapping(
        &tmp as *const StorageType as *const c_char,
        &mut val as *mut T as *mut c_char,
        size_of::<T>(),
      );
    } else {
      copy_nonoverlapping(
        buf.add(offset as usize),
        &mut val as *mut T as *mut c_char,
        size_of::<T>(),
      );
    }

    lua_pushnumber(l, val.to_f64());
    1
  }
}

fn static_assert_size<T, StorageType>() {
  assert!(size_of::<T>() == size_of::<StorageType>());
}

pub trait BufferReadableFloat {
  fn to_f64(self) -> f64;
}

impl BufferReadableFloat for f32 {
  fn to_f64(self) -> f64 {
    self as f64
  }
}

impl BufferReadableFloat for f64 {
  fn to_f64(self) -> f64 {
    self
  }
}
