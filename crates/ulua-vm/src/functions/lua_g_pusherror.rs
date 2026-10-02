use core::ffi::c_char;

use crate::{
  functions::{
    cstr_bytes_ref::cstr_bytes_ref, lua_rawcheckstack::lua_rawcheckstack,
    pusherror::pusherror_bytes,
  },
  records::lua_state::LuaState,
};

/// 安全字节切片版错误信息压栈。
///
/// r16-v7 步骤 2：同款接收者前移 `*mut` → `&mut LuaState`——体即 safe 引用形的
/// `lua_rawcheckstack` + 本票步骤 1 转 safe 的 `pusherror_bytes`，整体零 unsafe；
/// 检查栈先于压错的次序逐位不变。
pub(crate) fn lua_g_pusherror_bytes(l: &mut LuaState, error: &[u8]) {
  lua_rawcheckstack(l, 1);
  pusherror_bytes(l, error);
}

/// r16-v7 步骤 3（仿 v4c 判例）：导出垫片转 safe——`pub unsafe fn(*mut)` →
/// `pub fn(&mut LuaState, *const c_char)`，体一行转调本票 safe 化的
/// [`lua_g_pusherror_bytes`]，C 串裸参只经既有 pub(crate) safe 门面
/// [`cstr_bytes_ref`] 消费（裸参入 safe 实参位，`not_unsafe_ptr_arg_deref` 触发
/// 消亡；不新造门面、不动 `cstr_bytes` 本体）。检查栈先于压错的次序保持。
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`l` 存活与
/// 独占由 `&mut` 接收者类型承载；`error` 须为 NUL 结尾、调用期间存活的 C 串
/// （门面内 NUL 扫描必终止），消息拼装语义与 cpp 参考实现逐位不变。
pub fn lua_g_pusherror(l: &mut LuaState, error: *const c_char) {
  lua_g_pusherror_bytes(l, cstr_bytes_ref(error));
}
