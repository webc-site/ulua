use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setlvalue::setlvalue},
  records::lua_state::LuaState,
};

/// 对应 cpp `lapi.cpp:705-710` 的 `lua_pushinteger64`：以 `LUA_TINTEGER` 标签
/// 精确压入 i64（`setlvalue`），不经 f64、不丢精度。
///
/// `l` 以引用传入（存活由类型保证）：`ensure_stack(l,1)` 扩容后 `setlvalue!` 直写
/// top 槽并 `api_incr_top` 抬栈（写入槽合法性由该扩容与栈不变量保证）；
/// `ensure_stack` 可触发 GC。cpp VM/src/lapi.cpp:704
pub fn lua_pushinteger_64(l: &mut LuaState, n: i64) {
  // SAFETY: `l` 存活（引用形保证）；`ensure_stack(l,1)` 保证 top 槽在分配栈界内
  // 可写，`setlvalue!`/`api_incr_top!` 的其余前提由该扩容成立。
  unsafe {
    ensure_stack(l.as_mut_ptr(), 1);
    setlvalue!(l.top, n);
    api_incr_top!(l);
  }
}
