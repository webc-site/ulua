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
/// userdata、#2 为读取偏移（[`buffer_read_window_ref`] 取数据块并做 `size_of::<T>()`
/// 字节界校验，越界即抛错不返回）。结果数值压栈需栈顶余量。cpp lbuflib.cpp:67。
pub(crate) unsafe fn buffer_readinteger<T>(l: *mut LuaState) -> i32
where
  T: SwapBe + Into<f64>,
{
  // SAFETY: 契约保证窗口内偏移已界校验，装载只触达 size_of::<T>() 个可读字节
  unsafe {
    // r16-v17：`buffer_read_window_ref` 已收形为 `&mut LuaState`，本泛型核心的 C-ABI
    // 臂落在 luaopen_buffer.rs 的 `integer_wrappers!`（协议红线，本票不触碰），故形参
    // 暂保留裸 `*mut LuaState`，仅在转调窗口核心处一次 `&mut *l` 重建引用。
    let val = load_scalar_ref::<T>(buffer_read_window_ref(&mut *l, size_of::<T>()));

    (*l).push_number(val.into());
    1
  }
}
