use crate::{
  functions::{
    buffer_window::{buffer_data_len, buffer_data_ref, buffer_range_checked},
    lua_pushlstring::lua_pushlstring,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；窗口
/// 核心（buffer_data_len/buffer_range_checked/buffer_data_ref）自带 typeerror→oob
/// 抛错序，取参全部落在最终派窗之前故观察序不变（r16-p28 锚定形）；结果串压栈需栈顶余量。
pub(crate) fn buffer_readstring(l: &mut LuaState) -> i32 {
  let len = buffer_data_len(l, 1);
  let offset = l.check_integer(2);
  let size = l.check_integer(3);

  l.arg_check(size >= 0, 3, "size");

  // buffer_range_checked 的 isoutofbounds 已保证 [offset, offset+size) 落在 buffer 数据界内
  let (start, end) = buffer_range_checked(l, len, offset, size as usize);
  let region = &buffer_data_ref(l, 1)[start..end];
  // 锚定形：窗口借用止于 (ptr, len) 快照，其后 `l` 恢复可用
  let (ptr, n) = (region.as_ptr(), region.len());

  // SAFETY: `l.as_mut_ptr()` 为由借用重建的存活 C 函数帧裸参、处于可 GC/可分配帧；
  // `ptr`/`n` 快照自界内 buffer 窗口（契约三要素见 lua_tobuffer_bytes_ref），核心只
  // 界内拷入堆上 TString（契约见 lua_pushlstring）。
  unsafe { lua_pushlstring(l.as_mut_ptr(), ptr.cast(), n) };

  1
}

lua_lib_fn!(pub(crate) fn buffer_readstring @ref, buffer_readstring_arm);
