use core::{
  mem::{size_of, zeroed},
  ptr::{addr_of, addr_of_mut, copy_nonoverlapping, read_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{assert_same_width, buffer_read_window},
    lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata、#2 为读取偏移（[`buffer_read_window`] 做 `size_of::<T>()` 字节界校验，
/// 越界即抛错不返回）。结果数值压栈需栈顶余量。cpp lbuflib.cpp:147 `buffer_readfp`。
pub(crate) unsafe fn buffer_readfp<T, StorageType>(l: *mut LuaState) -> i32
where
  T: Copy + BufferReadableFloat,
  StorageType: SwapBe,
{
  // Safety: 窗口已挡下越界偏移（失败即抛错不返回），界内浮点字节数可读
  unsafe {
    let src = buffer_read_window(l, size_of::<T>());

    // 字节按位拷入本地值：read_unaligned 覆盖 buf 的对齐不确定性，与 cpp memcpy 逐位等价
    let val: T = if LUAU_BIG_ENDIAN {
      assert_same_width::<T, StorageType>();
      let mut tmp: StorageType = zeroed();
      copy_nonoverlapping(
        src,
        addr_of_mut!(tmp).cast::<u8>(),
        size_of::<StorageType>(),
      );
      let tmp = tmp.swap_be();
      read_unaligned(addr_of!(tmp).cast::<T>())
    } else {
      read_unaligned(src.cast::<T>())
    };

    lua_pushnumber(l, val.to_f64());
    1
  }
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
