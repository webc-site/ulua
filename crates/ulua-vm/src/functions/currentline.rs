use core::ptr::addr_of;

use crate::{
  functions::{currentpc::currentpc, lua_g_getline::lua_g_getline},
  macros::ci_func::ci_func,
  records::{call_info::CallInfo, closure::LClosure},
};

/// 行号反查（cpp `VM/src/ldebug.cpp:28`）：经帧槽 `ci.func` 取当前帧 `Proto`，按
/// [`currentpc`] 折出的 PC 交 [`lua_g_getline`] 查行。纯只读，不分配、不抛错。
///
/// 契约（调用方保证）：`ci` 为存活 `CallInfo` 且为 Lua 帧（`isLua!` 成立）——`ci.func`
/// 指向存活 `TValue` 槽，其 `inner.l` 内嵌 `LClosure.p` 须指向存活 `Proto`，且
/// `lineinfo`/`abslineinfo` 与 `sizecode` 自洽（`currentpc`/`lua_g_getline` 的前置条件即此）。
pub(crate) fn currentline(ci: &CallInfo) -> i32 {
  // SAFETY: 契约保证帧槽闭包与 `Proto` 存活（同 [`currentpc`]），块内仅取 `p` 裸指针并降
  // 共享引用透传给纯读函数；行号越界读已由 `lua_g_getline` 的切片 `get` 折叠兜底。
  unsafe {
    let cl = ci_func!(ci);
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    lua_g_getline(&*(*lcl).p, currentpc(ci))
  }
}
