use core::{
  ffi::c_int,
  mem::size_of,
  ptr::{copy_nonoverlapping, read_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::buffer_swapbe, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checknumber::lua_l_checknumber,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn buffer_writefp<T, StorageType>(l: *mut lua_State) -> c_int
where
  T: Copy + BufferFloat,
  StorageType: Copy,
{
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);

    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checknumber(l, 3);

    if isoutofbounds(offset, len, size_of::<T>()) {
      luaL_error!(l, "buffer access out of bounds");
    }

    let val: T = T::from_f64(value);

    if LUAU_BIG_ENDIAN {
      static_assert_size_match::<T, StorageType>();
      let mut tmp: StorageType = read_unaligned(&val as *const T as *const StorageType);
      tmp = buffer_swapbe(tmp);
      copy_nonoverlapping(
        &tmp as *const StorageType as *const u8,
        (buf as *mut u8).add(offset as usize),
        size_of::<StorageType>(),
      );
    } else {
      copy_nonoverlapping(
        &val as *const T as *const u8,
        (buf as *mut u8).add(offset as usize),
        size_of::<T>(),
      );
    }

    0
  }
}

// Helper to enforce sizeof(T) == sizeof(StorageType) at compile time
fn static_assert_size_match<T, StorageType>() {
  assert!(size_of::<T>() == size_of::<StorageType>());
}

pub trait BufferFloat {
  fn from_f64(value: f64) -> Self;
}

impl BufferFloat for f32 {
  fn from_f64(value: f64) -> Self {
    value as f32
  }
}

impl BufferFloat for f64 {
  fn from_f64(value: f64) -> Self {
    value
  }
}
