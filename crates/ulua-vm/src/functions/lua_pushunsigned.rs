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
///
/// r12-w4b 消费面实测：调用点全在 `ulua-vm` 内——binary32 族 `b_replace`/`b_extract`/
/// `b_rot`/`b_shift`/`b_arshift`/`bit_map1`/`bitfold` 与 `buffer_readbits` 共 8 处，
/// 跨 crate、`ulua-capi` 导出面与 conformance/rt 测试门面实测零消费，故按 §7 零死代码
/// 不保留 `*mut LuaState` 形垫片。若将来按 cpp 开 `ulua_lua_pushunsigned` 导出，循
/// `lua_pushinteger_64` 的显式壳先例在 capi 侧一行折形即可。
pub(crate) fn lua_pushunsigned(l: &mut LuaState, u: c_uint) {
  l.push_number(f64::from(u));
}
