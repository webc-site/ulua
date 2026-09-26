use core::{
  mem::size_of,
  ptr::{addr_of, copy_nonoverlapping, read_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{assert_same_width, buffer_at, buffer_data},
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checknumber::lua_l_checknumber,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata（[`buffer_data`] 取其数据块与长度），#2 为写入偏移，#3 为有限数值
/// （inf/nan 由 `lua_l_checknumber` 报错挡下）；偏移越界经 [`buffer_at`] 抛错、
/// 不返回，`size_of::<T>()` 字节写入界由该检查收口。cpp lbuflib.cpp:174 `buffer_writefp`。
pub(crate) unsafe fn buffer_writefp<T, StorageType>(l: *mut LuaState) -> i32
where
  T: Copy + BufferFloat,
  StorageType: SwapBe,
{
  // Safety: 契约保证 #1 为 buffer、界校验后 `size_of::<T>()` 字节可写（越界即抛错不返回）
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checknumber(l, 3);

    let dst = buffer_at(l, buf, len, offset, size_of::<T>());
    let val: T = T::from_f64(value);

    if LUAU_BIG_ENDIAN {
      assert_same_width::<T, StorageType>();
      let tmp: StorageType = read_unaligned(addr_of!(val).cast::<StorageType>());
      let tmp = tmp.swap_be();
      copy_nonoverlapping(addr_of!(tmp).cast::<u8>(), dst, size_of::<StorageType>());
    } else {
      copy_nonoverlapping(addr_of!(val).cast::<u8>(), dst, size_of::<T>());
    }

    0
  }
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
