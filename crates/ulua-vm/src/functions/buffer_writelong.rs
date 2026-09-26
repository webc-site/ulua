use core::mem::size_of;

use crate::{
  functions::{
    buffer_window::{buffer_at, buffer_data, store_scalar},
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkinteger_64::lua_l_checkinteger_64,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`：#1 为 buffer userdata、#2 为写入偏移、
/// #3 为待写入整数；偏移越界经 [`buffer_at`] 抛错、不返回（8 字节写入界由该检查收口）。
pub(crate) unsafe fn buffer_writelong(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 buffer 内该偏移起 8 字节可写，i64 序列化写入不越出数据界
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkinteger_64(l, 3);

    store_scalar(buffer_at(l, buf, len, offset, size_of::<i64>()), value);

    0
  }
}

lua_lib_fn!(pub(crate) fn buffer_writelong, buffer_writelong_arm);
