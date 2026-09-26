use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::SwapBe,
    buffer_window::{buffer_read_window, load_scalar},
    lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata、#2 为读取偏移（[`buffer_read_window`] 取数据块并做 `size_of::<T>()`
/// 字节界校验，越界即抛错不返回）。结果数值压栈需栈顶余量。cpp lbuflib.cpp:67。
pub(crate) unsafe fn buffer_readinteger<T>(l: *mut LuaState) -> i32
where
  T: SwapBe + Into<f64>,
{
  // Safety: 契约保证窗口内偏移已界校验，装载只触达 size_of::<T>() 个可读字节
  unsafe {
    let val = load_scalar::<T>(buffer_read_window(l, size_of::<T>()));

    lua_pushnumber(l, val.into());
    1
  }
}
