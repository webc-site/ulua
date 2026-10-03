use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_data_len, buffer_data_ref, buffer_range_checked, store_scalar_ref},
  },
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；栈槽 #1 为 buffer
/// userdata（[`buffer_data_len`] 先行取景并落 typeerror），#2 为写入偏移，#3 为有限
/// 数值（inf/nan 由 `lua_l_checknumber` 报错挡下，取参不跑元方法）；偏移越界经
/// [`buffer_range_checked`] 抛错、不返回，`size_of::<T>()` 字节写入界由该检查收口；
/// 数据窗末派直达写入（r16-p28 锚定形）。浮点回写塌缩为 [`store_scalar_ref`] 单点：端序由 `SwapBe`
/// （buffer_swapbe.rs，f32/f64 经 `to_bits`/`from_bits` 按整型位宽翻转，cpp 大端
/// 三件套 `static_cast<StorageType>` 重排的逐位等价形）承载。cpp lbuflib.cpp:174
/// `buffer_writefp`。
pub(crate) fn buffer_writefp<T>(l: &mut LuaState) -> i32
where
  T: BufferFloat,
{
  let len = buffer_data_len(l, 1);
  let offset = l.check_integer(2);
  let value = l.check_number(3);
  let (start, end) = buffer_range_checked(l, len, offset, size_of::<T>());

  let val: T = T::from_f64(value);
  store_scalar_ref(&mut buffer_data_ref(l, 1)[start..end], val);

  0
}

// review.md §7：本项无 crate 外消费，由 pub 收窄为 pub(crate)。
// 父 trait `SwapBe` 已含 `Copy`，即 store_scalar_ref 的密封元素前提；
// `from_f64` 对应 cpp `T val = T(value)` 的数值 narrowing（重排前完成，同序）。
pub(crate) trait BufferFloat: SwapBe {
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
