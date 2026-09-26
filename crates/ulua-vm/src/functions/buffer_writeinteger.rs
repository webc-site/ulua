use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::BufferInt,
    buffer_window::{buffer_at, buffer_data, store_scalar},
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata（[`buffer_data`] 取其数据块与长度），#2 为写入偏移，#3 为待写入整数
/// （`lua_l_checkunsigned` 截断到位宽）；偏移越界经 [`buffer_at`] 抛错、不返回。
/// cpp lbuflib.cpp:88 `buffer_writeinteger`。
pub(crate) unsafe fn buffer_writeinteger<T>(l: *mut LuaState) -> i32
where
  T: BufferInt,
{
  // Safety: 契约保证 #1 为 buffer、#2 为界内偏移（越界即抛错不返回），界内 1..4 字节可写
  unsafe {
    let (buf, len) = buffer_data(l, 1);
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkunsigned(l, 3);

    // cpp `T val = T(value)`：数值截断，端序无关
    store_scalar(
      buffer_at(l, buf, len, offset, size_of::<T>()),
      T::from_u32_trunc(value),
    );

    0
  }
}
