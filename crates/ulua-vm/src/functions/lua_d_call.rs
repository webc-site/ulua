use crate::{
  functions::lua_d_callint::lua_d_callint, records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

// plain `unsafe fn` (not extern "C"): Lua errors unwind through here via
// panic/catch_unwind, and extern "C" frames abort on unwind
/// # Safety
/// `l` 须为存活 `LuaState` 且调用点已在 `luaD_checkstack`/pcall 保护帧内：`func` 须为栈内合法 `StkId`，
/// 其处函数与 `func+1..(*l).top` 的实参须存活，`nresults` 为约定结果数（含 LUA_MULTRET）；委托 `lua_d_callint(..,false)`，
/// Lua 错误经 unwind 穿出本非 extern 帧（故声明为 plain `unsafe fn`）。
/// cpp VM/src/ldo.cpp:382
pub unsafe fn lua_d_call(l: *mut LuaState, func: StkId, nresults: i32) {
  unsafe {
    lua_d_callint(l, func, nresults, false);
  }
}
