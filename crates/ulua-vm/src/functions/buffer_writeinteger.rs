use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::BufferInt,
    buffer_window::{buffer_data_len, buffer_data_ref, buffer_range_checked, store_scalar_ref},
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；栈槽 #1 为 buffer
/// userdata（[`buffer_data_len`] 先行取景并落 typeerror），#2 为写入偏移，#3 为待写入
/// 整数（`lua_l_checkunsigned` 截断到位宽，取参不跑元方法）；偏移越界经
/// [`buffer_range_checked`] 抛错、不返回；数据窗末派直达 [`store_scalar_ref`] 写入
/// （r16-p28 锚定形）。cpp lbuflib.cpp:88 `buffer_writeinteger`。
pub(crate) fn buffer_writeinteger<T>(l: &mut LuaState) -> i32
where
  T: BufferInt,
{
  let len = buffer_data_len(l, 1);
  let offset = l.check_integer(2);
  let value = lua_l_checkunsigned(l, 3);
  let (start, end) = buffer_range_checked(l, len, offset, size_of::<T>());

  // cpp `T val = T(value)`：数值截断，端序无关
  store_scalar_ref(
    &mut buffer_data_ref(l, 1)[start..end],
    T::from_u32_trunc(value),
  );

  0
}
