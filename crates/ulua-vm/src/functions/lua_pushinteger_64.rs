use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setlvalue::setlvalue},
  records::lua_state::LuaState,
};

/// 对应 cpp `lapi.cpp:705-710` 的 `lua_pushinteger64`：以 `LUA_TINTEGER` 标签
/// 精确压入 i64（`setlvalue`），不经 f64、不丢精度。
/// # Safety
/// `l` 须为存活 `LuaState`：`ensure_stack(l,1)` 扩容后 `setlvalue!((*l).top,n)` 直写 top 槽并 `api_incr_top` 抬栈
/// （故写入前 `(*l).top` 须为栈内合法可写槽）；`n: i64` 以 LUA_TINTEGER 精确压入不经 f64。`ensure_stack` 可触发 GC。
/// cpp VM/src/lapi.cpp:704
pub unsafe fn lua_pushinteger_64(l: *mut LuaState, n: i64) {
  unsafe {
    ensure_stack(l, 1);
    setlvalue!((*l).top, n);
    api_incr_top!(l);
  }
}
