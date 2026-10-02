use crate::{
  macros::{api_incr_top::api_incr_top, setlvalue::setlvalue},
  records::lua_state::LuaState,
};

/// 对应 cpp `lapi.cpp:705-710` 的 `lua_pushinteger64`：以 `LUA_TINTEGER` 标签
/// 精确压入 i64（`setlvalue`），不经 f64、不丢精度。
///
/// `l` 以引用传入（存活由类型保证）：栈余量先经 B 族窄腰方法
/// `LuaState::ensure_stack_space(1)`（即 `ensure_stack(l, 1)`，可移动栈、可触发 GC），
/// 扩容之后才派生本槽写面——`setlvalue!` 直写 top 槽、`api_incr_top!` 抬栈顶，
/// 两步的合法性由该扩容与栈不变量保证。
///
/// 与 [`LuaState::push_number`] 一侧的 push 族基元不同，本函数的 `LUA_TINTEGER` 精确槽
/// 尚无签名安全的串接面（`push_slot_with` 收在 `records/lua_state/stack.rs` 私有原语内，
/// 上收出本票文件集），故保留 2 语句窄 unsafe 窗，登记为后续「栈槽句柄」小票建议。
pub fn lua_pushinteger_64(l: &mut LuaState, n: i64) {
  // 扩容先行（可移动栈），其后不留裸指针算式
  l.ensure_stack_space(1);

  // SAFETY: `l` 存活（引用形保证）；上一步 `ensure_stack_space(1)` 已保证 top 槽在分配
  // 栈界内可独占写入，`setlvalue!`/`api_incr_top!` 的其余前提由该扩容与栈不变量成立
  unsafe {
    setlvalue!(l.top, n);
    api_incr_top!(l);
  }
}
