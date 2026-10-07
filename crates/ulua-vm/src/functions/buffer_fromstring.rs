use core::slice::from_raw_parts;

use crate::{
  functions::{
    buffer_window::buffer_data_ref, lua_l_checklstring::lua_l_checklstring_ref,
    lua_newbuffer::lua_newbuffer_push_ref,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// cpp `buffer_fromstring`（lbuflib.cpp:47）：按字符串实参长度新建 buffer 并整段复制。
///
/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载且处于受保护
/// 帧（`checklstring` 非串实参经 `tag_error` 抛 "string expected" 发散），栈顶有压入新
/// buffer 的余量。重入面论证：仅 #1 一次取参，走 `lua_tolstring`→`luaV_tostring` 内置
/// 数值转换（cpp lvmutils.cpp:39，不调 `__tostring`、不跑 Lua 代码）；`lua_newbuffer_push_ref`
/// 只做分配/GC 步进/挪栈（Lua 串与 buffer 对象均不移动，#1 串切片按 `lua_l_checklstring_ref`
/// 契约在本次调用内可读），其后到写入点之间不再有任何取参或可抛错重入路径——新对象在栈顶
/// `-1` 槽钉住、定长不 resize，数据窗经 `buffer_data_ref` 后置派生直达 `copy_from_slice`，
/// 源与目的分属两个对象不重叠。
pub(crate) fn buffer_fromstring(l: &mut LuaState) -> i32 {
  let val = lua_l_checklstring_ref(l, 1);
  // 锚定形：串窗借用止于 (ptr, len) 快照（Lua 串不可变不移动、#1 槽引用钉住存活，
  // 快照在写入点仍有效），其后 `l` 恢复可用
  let (vptr, vlen) = (val.as_ptr(), val.len());

  // 新 buffer 压入栈顶；返回裸数据指针面已随 r12-w6b 收窄消灭，数据窗经窄腰切片形取回
  lua_newbuffer_push_ref(l, vlen);
  // 刚压入的槽必是 buffer（typeerror 分支不可达），数据界即新建长度 `vlen`
  let dst = buffer_data_ref(l, -1);
  debug_assert_eq!(dst.len(), vlen);

  // SAFETY: 源视图由 #1 串窗的 (ptr, len) 快照重物化——串不可变不移动、栈槽引用钉住
  // （同 checklstring 切片契约；newbuffer 分配/挪栈不搬移串对象）；目的窗为新建 buffer
  // 全长，两区间长度相等且分属两个对象不重叠
  unsafe { dst.copy_from_slice(from_raw_parts(vptr, vlen)) };

  1
}

lua_lib_fn!(pub(crate) fn buffer_fromstring @ref, buffer_fromstring_arm);
