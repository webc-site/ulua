use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_read_window_ref, load_scalar_ref},
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata、#2 为读取偏移（[`buffer_read_window_ref`] 做 `size_of::<T>()` 字节界校验，
/// 越界即抛错不返回）。结果数值压栈需栈顶余量。浮点取窗塌缩为
/// [`load_scalar_ref`] 单点：端序由 `SwapBe`（buffer_swapbe.rs，f32/f64 经
/// `to_bits`/`from_bits` 按整型位宽翻转，cpp 大端三件套 `static_cast<StorageType>`
/// 重排的逐位等价形）承载。cpp lbuflib.cpp:147 `buffer_readfp`。
pub(crate) unsafe fn buffer_readfp<T>(l: *mut LuaState) -> i32
where
  T: BufferReadableFloat,
{
  // SAFETY: 窗口已挡下越界偏移（失败即抛错不返回），界内浮点字节数可读；
  // 抛错序与旧形逐位不变（先窗口校验、后压栈）
  unsafe {
    let val = load_scalar_ref::<T>(buffer_read_window_ref(l, size_of::<T>()));

    (*l).push_number(val.to_f64());
    1
  }
}

// review.md §7：本项无 crate 外消费，由 pub 收窄为 pub(crate)。
// 父 trait `SwapBe` 已含 `Copy`，即 load_scalar_ref 的密封元素前提；
// f32/f64 的 `to_f64` 对应 cpp `double(val)` 的数值 widening（重排完成后才发生，
// 与大端分支 memcpy 回 T 再 cast 同序）。
pub(crate) trait BufferReadableFloat: SwapBe {
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
