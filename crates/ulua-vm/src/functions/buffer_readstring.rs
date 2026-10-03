use crate::{
  functions::{
    buffer_window::{buffer_at_ref, buffer_data_ref},
    lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；窗口切片
/// 核心（buffer_data_ref/buffer_at_ref）自带 typeerror→oob 抛错序，派生点保持在取参
/// 之前故观察序不变；结果串压栈需栈顶余量。
pub(crate) fn buffer_readstring(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  let offset = l.check_integer(2);
  let size = l.check_integer(3);

  l.arg_check(size >= 0, 3, "size");

  // buffer_at_ref 的 isoutofbounds 已保证 [offset, offset+size) 落在 buffer 数据界内
  let region = buffer_at_ref(l, buf, offset, size as usize);

  // SAFETY: `l` 为借用重建的存活 C 函数帧、处于可 GC/可分配帧，`region` 为界内切片
  // 借用，核心只界内拷入堆上 TString（契约见 lua_pushlstring_bytes）。
  unsafe { lua_pushlstring_bytes(l, region) };

  1
}

lua_lib_fn!(pub(crate) fn buffer_readstring @ref, buffer_readstring_arm);
