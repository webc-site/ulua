use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_at_ref, buffer_data_ref, store_scalar_ref},
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata（[`buffer_data_ref`] 取其数据块借用），#2 为写入偏移，#3 为有限数值
/// （inf/nan 由 `lua_l_checknumber` 报错挡下，取参不跑元方法，先行窗口借用无再入
/// 重叠）；偏移越界经 [`buffer_at_ref`] 抛错、不返回，`size_of::<T>()` 字节写入界由
/// 该检查收口。浮点回写塌缩为 [`store_scalar_ref`] 单点：端序由 `SwapBe`
/// （buffer_swapbe.rs，f32/f64 经 `to_bits`/`from_bits` 按整型位宽翻转，cpp 大端
/// 三件套 `static_cast<StorageType>` 重排的逐位等价形）承载。cpp lbuflib.cpp:174
/// `buffer_writefp`。
pub(crate) unsafe fn buffer_writefp<T>(l: *mut LuaState) -> i32
where
  T: BufferFloat,
{
  // SAFETY: 契约保证 #1 为 buffer、界校验后 `size_of::<T>()` 字节可写（越界即抛错不返回）；
  // 校验/取参/抛错序与旧形逐位不变（先 check 后写）
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let offset = (*l).check_integer(2);
    let value = (*l).check_number(3);

    let dst = buffer_at_ref(l, buf, offset, size_of::<T>());
    let val: T = T::from_f64(value);

    store_scalar_ref(dst, val);

    0
  }
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
