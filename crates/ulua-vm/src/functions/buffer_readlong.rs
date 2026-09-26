use core::mem::size_of;

use crate::{
  functions::{
    buffer_window::{buffer_read_window, load_scalar},
    lua_pushinteger_64::lua_pushinteger_64,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：#1 为 buffer userdata、#2 为
/// 读取偏移（[`buffer_read_window`] 做 `size_of::<u64>()` 字节界校验，越界即抛错不返回）。
/// 结果压栈需栈顶余量。cpp lbuflib.cpp `buffer_readlong`。
pub(crate) unsafe extern "C-unwind" fn buffer_readlong(l: *mut LuaState) -> i32 {
  // Safety: 契约保证窗口内偏移已界校验，装载只触达 8 个可读字节
  unsafe {
    let val = load_scalar::<u64>(buffer_read_window(l, size_of::<u64>()));

    lua_pushinteger_64(l, val as i64);
    1
  }
}
