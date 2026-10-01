use core::ptr::addr_of;

use crate::{
  macros::{ci_func::ci_func, pc_rel::pcRel},
  records::{call_info::CallInfo, closure::LClosure},
};

/// PC 反查（cpp `VM/src/ldebug.cpp:24` `currentPC`）：经帧槽 `ci.func` 取 Lua 闭包原型，
/// 以 `savedpc` 相对 `code` 基址折出 0 基指令下标。纯只读，不分配、不抛错。
///
/// 契约（调用方按帧链存活不变量保证）：`ci` 为存活 `CallInfo` 且为 Lua 帧——`ci.func`
/// 指向存活 `TValue` 槽且其 payload 为 `Closure`（`isLua!` 成立），`inner.l.p` 为存活
/// `Proto`；非 Lua 帧调用即 union 误读。
pub(crate) fn currentpc(ci: &CallInfo) -> i32 {
  // SAFETY: 契约保证 `ci.func` 槽存活且为函数 TValue（调用方 isLua!/等价判定先行），
  // `inner.l.p` 为存活 Proto；块内仅做只读取指针，不落任何写。
  unsafe {
    let cl = ci_func!(ci);
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    pcRel!(ci.savedpc, (*lcl).p)
  }
}
