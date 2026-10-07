use crate::{
  functions::{buffer_window::buffer_data_ref, lua_pushlstring::lua_pushlstring},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载，实参 1
/// 为已检查 buffer userdata；buffer 全长拷回的串压栈需栈顶余量。r16-p28 锚定形：
/// 窗口借用止于 (ptr, len) 快照，快照指向的 buffer 内联数据块由栈槽引用钉住。
pub(crate) fn buffer_tostring(l: &mut LuaState) -> i32 {
  let data = buffer_data_ref(l, 1);

  // 锚定形：窗口借用止于 (ptr, len) 快照，其后 `l` 恢复可用
  let (ptr, len) = (data.as_ptr(), data.len());

  // SAFETY: `l.as_mut_ptr()` 为由借用重建的存活 C 函数帧裸参；`ptr`/`len` 快照自
  // 栈槽 buffer 窗口（buffer 定长不 resize、GC 不移动、栈槽引用钉住——契约三要素见
  // lua_tobuffer_bytes_ref），核心只界内拷入堆上 TString（契约见 lua_pushlstring）。
  unsafe { lua_pushlstring(l.as_mut_ptr(), ptr.cast(), len) };

  1
}

lua_lib_fn!(pub(crate) fn buffer_tostring @ref, buffer_tostring_arm);
