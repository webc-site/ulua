use core::ffi::c_uint;

use crate::records::lua_state::LuaState;

/// 对应 cpp `lapi.cpp` 的 `lua_pushunsigned`：`ensure_stack(L,1)` →
/// `setnvalue(L->top, cast_num(u))` → `api_incr_top(L)`。
///
/// 三步在 `&mut LuaState` 的 push 族方法上串接（r12-w4b，参照 r12 已并的 lua_r_newclass
/// `push_nil` 形）：[`LuaState::push_number`] 体内即「扩容 → 保留 top 槽 `set_nvalue` →
/// 抬栈顶」的单一收口原语 `push_slot_with`，求值次序与 cpp 逐指令一致（扩容先行、
/// 借用后派生）。u32 → f64 无损（尾数 52 位 > 32 位），与 cpp `cast_num(u)` 同值。
///
/// 本文件不再有 `unsafe`：`l` 的存活由引用类型保证，槽写入面收在 `push_slot_with`
/// 的有契约最小边界内（`records/lua_state/stack.rs`）。
pub(crate) fn lua_pushunsigned(l: &mut LuaState, u: c_uint) {
  l.push_number(f64::from(u));
}
