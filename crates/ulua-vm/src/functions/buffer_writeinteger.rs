use core::mem::size_of;

use crate::{
  functions::{
    buffer_swapbe::BufferInt,
    buffer_window::{buffer_at_ref, buffer_data_ref, store_scalar_ref},
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 buffer 库 C 函数帧的存活 `LuaState`：栈槽 #1 为 buffer
/// userdata（[`buffer_data_ref`] 取其数据块借用），#2 为写入偏移，#3 为待写入整数
/// （`lua_l_checkunsigned` 截断到位宽，取参不跑元方法，先行窗口借用无再入重叠）；
/// 偏移越界经 [`buffer_at_ref`] 抛错、不返回。cpp lbuflib.cpp:88 `buffer_writeinteger`。
pub(crate) unsafe fn buffer_writeinteger<T>(l: *mut LuaState) -> i32
where
  T: BufferInt,
{
  // SAFETY: 契约保证 #1 为 buffer、#2 为界内偏移（越界即抛错不返回），界内
  // size_of::<T>() 字节可写；#3 取参不执行 Lua 代码，借用跨取参存活合规
  unsafe {
    let buf = buffer_data_ref(l, 1);
    let offset = (*l).check_integer(2);
    let value = lua_l_checkunsigned(&mut *l, 3);

    // cpp `T val = T(value)`：数值截断，端序无关
    store_scalar_ref(
      buffer_at_ref(l, buf, offset, size_of::<T>()),
      T::from_u32_trunc(value),
    );

    0
  }
}
