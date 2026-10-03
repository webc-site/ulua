use core::mem::size_of;

use crate::{
  functions::buffer_window::{buffer_at_ref, buffer_data_ref, store_scalar_ref},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；#1 为 buffer
/// userdata、#2 为写入偏移、#3 为待写入整数；偏移越界经 [`buffer_at_ref`] 抛错、不返回
/// （8 字节写入界由该检查收口）。
pub(crate) fn buffer_writelong(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  let offset = l.check_integer(2);
  let value = l.check_integer_64(3);

  store_scalar_ref(buffer_at_ref(l, buf, offset, size_of::<i64>()), value);

  0
}

lua_lib_fn!(pub(crate) fn buffer_writelong @ref, buffer_writelong_arm);
