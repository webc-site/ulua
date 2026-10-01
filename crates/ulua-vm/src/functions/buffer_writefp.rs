use core::{
  mem::size_of,
  ptr::{addr_of, copy_nonoverlapping, read_unaligned},
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{assert_same_width, buffer_at_ref, buffer_data_ref},
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata（[`buffer_data_ref`] 取其数据块借用），#2 为写入偏移，#3 为有限数值
/// （inf/nan 由 `lua_l_checknumber` 报错挡下，取参不跑元方法，先行窗口借用无再入
/// 重叠）；偏移越界经 [`buffer_at_ref`] 抛错、不返回，`size_of::<T>()` 字节写入界由
/// 该检查收口。浮点回写仍走 `SwapBe` 三件套原形（`SwapBe` 密封清单无 f32/f64，
/// buffer_swapbe.rs 出本票范围，浮点收口缓发登记至 T9 裁决）。cpp lbuflib.cpp:174
/// `buffer_writefp`。
pub(crate) unsafe fn buffer_writefp<T, StorageType>(l: *mut LuaState) -> i32
where
  T: Copy + BufferFloat,
  StorageType: SwapBe,
{
  // SAFETY: 契约保证 #1 为 buffer、界校验后 `size_of::<T>()` 字节可写（越界即抛错不返回）
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let offset = (*l).check_integer(2);
    let value = (*l).check_number(3);

    let dst = buffer_at_ref(l, buf, offset, size_of::<T>());
    let val: T = T::from_f64(value);

    if LUAU_BIG_ENDIAN {
      assert_same_width::<T, StorageType>();
      let tmp: StorageType = read_unaligned(addr_of!(val).cast::<StorageType>());
      let tmp = tmp.swap_be();
      // SAFETY: buffer_at_ref 保证 dst 起 size_of::<T>() 字节可写且
      // assert_same_width 已令其等于 size_of::<StorageType>()；copy 源为栈上 tmp
      copy_nonoverlapping(
        addr_of!(tmp).cast::<u8>(),
        dst.as_mut_ptr(),
        size_of::<StorageType>(),
      );
    } else {
      // SAFETY: buffer_at_ref 保证 dst 起 size_of::<T>() 字节可写；copy 源为栈上 val
      copy_nonoverlapping(addr_of!(val).cast::<u8>(), dst.as_mut_ptr(), size_of::<T>());
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
