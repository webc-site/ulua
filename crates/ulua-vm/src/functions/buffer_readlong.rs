use core::mem::size_of;

use crate::{
  functions::{
    buffer_window::{buffer_read_window_ref, load_scalar_ref},
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；#1 为
/// buffer userdata、#2 为读取偏移（[`buffer_read_window_ref`] 做 `size_of::<u64>()`
/// 字节界校验，越界即抛错不返回）；结果压栈需栈顶余量。cpp lbuflib.cpp `buffer_readlong`。
pub(crate) fn buffer_readlong(l: &mut LuaState) -> i32 {
  let val = load_scalar_ref::<u64>(buffer_read_window_ref(l, size_of::<u64>()));

  lua_pushinteger_64(l, val as i64);
  1
}

lua_lib_fn!(pub(crate) fn buffer_readlong @ref, buffer_readlong_arm);
