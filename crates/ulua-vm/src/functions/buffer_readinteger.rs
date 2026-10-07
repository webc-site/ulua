use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_read_window_ref, load_scalar_ref},
  },
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；栈槽 #1 为 buffer
/// userdata、#2 为读取偏移（[`buffer_read_window_ref`] 取数据块并做 `size_of::<T>()`
/// 字节界校验，越界即抛错不返回）。结果数值压栈需栈顶余量。cpp lbuflib.cpp:67。
pub(crate) fn buffer_readinteger<T>(l: &mut LuaState) -> i32
where
  T: SwapBe + Into<f64>,
{
  let val = load_scalar_ref::<T>(buffer_read_window_ref(l, size_of::<T>()));

  l.push_number(val.into());
  1
}
