use crate::{
  functions::{buffer_window::buffer_data_ref, lua_pushlstring::lua_pushlstring_bytes},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载，实参 1
/// 为已检查 buffer userdata；buffer 全长拷回的串压栈需栈顶余量。
pub(crate) fn buffer_tostring(l: &mut LuaState) -> i32 {
  let data = buffer_data_ref(l, 1);

  // SAFETY: `l` 为借用重建的存活 C 函数帧、处于可 GC/可分配帧，`data` 为界内 buffer
  // 切片借用；核心只界内拷入堆上 TString、不留存借出（契约见 lua_pushlstring_bytes）。
  unsafe { lua_pushlstring_bytes(l, data) };

  1
}

lua_lib_fn!(pub(crate) fn buffer_tostring @ref, buffer_tostring_arm);
