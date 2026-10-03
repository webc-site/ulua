use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::BufferInt,
    buffer_window::{buffer_at_ref, buffer_data_ref, store_scalar_ref},
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；栈槽 #1 为 buffer
/// userdata（[`buffer_data_ref`] 取其数据块借用），#2 为写入偏移，#3 为待写入整数
/// （`lua_l_checkunsigned` 截断到位宽，取参不跑元方法，先行窗口借用无再入重叠）；
/// 偏移越界经 [`buffer_at_ref`] 抛错、不返回。cpp lbuflib.cpp:88 `buffer_writeinteger`。
pub(crate) fn buffer_writeinteger<T>(l: &mut LuaState) -> i32
where
  T: BufferInt,
{
  let buf = buffer_data_ref(l, 1);
  let offset = l.check_integer(2);
  let value = lua_l_checkunsigned(l, 3);

  // cpp `T val = T(value)`：数值截断，端序无关
  store_scalar_ref(
    buffer_at_ref(l, buf, offset, size_of::<T>()),
    T::from_u32_trunc(value),
  );

  0
}
