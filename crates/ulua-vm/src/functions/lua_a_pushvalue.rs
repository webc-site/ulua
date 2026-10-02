use crate::{records::lua_state::LuaState, type_aliases::t_value::TValue};

/// 按值压入 TValue（`luaA_pushvalue`）：`l` 以引用传入（存活由类型保证）；`o` 以
/// 引用传入，整槽按值拷贝写入保留顶槽。收编入 [`LuaState::push_slot_with`] 单一
/// 收口原语——扩容先行（`ensure_stack` 可能移动栈，故其后再派生本槽独占借用）、
/// 写槽、`api_incr_top` 抬顶的求值次序与旧逐指令形态一致。cpp `lapi.cpp:143`。
pub fn lua_a_pushvalue(l: &mut LuaState, o: &TValue) {
  l.push_slot_with(|slot| *slot = *o);
}
