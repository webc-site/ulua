//! `lua_a_pushclass` — push a `LuauClass*` value onto the Lua stack.
//! C++ source: `VM/src/lapi.cpp:133`

use crate::{
  enums::lua_type::LuaType,
  functions::ensure_stack::ensure_stack,
  macros::{api_check::api_check, api_incr_top::api_incr_top},
  records::{gc_object::GCObject, lua_state::LuaState, luau_class::LuauClass},
};

/// # Safety
/// `l` 须为存活 LuaState 且处于可分配/GC 的受保护帧，`ensure_stack(l, 1)` 保证 `(*l).top` 之后至少留 1 个空槽
/// （写入 class TValue 并 `api_incr_top` 上移栈顶）；`lco` 须为非空且存活的 LuauClass（`api_check` 仅 debug 断言，
/// release 不校验，故非空由调用方保证）。cpp/VM/src/lapi.cpp:150 luaA_pushclass。
pub unsafe fn lua_a_pushclass(l: *mut LuaState, lco: *mut LuauClass) {
  unsafe {
    // cpp `ensure_stack(L, 1)` 在 api_check 之前
    ensure_stack(l, 1);
    api_check!(l, !lco.is_null());

    let i_o = (*l).top;
    (*i_o).value.gc = lco as *mut GCObject;
    (*i_o).set_tt(LuaType::Class as i32);

    api_incr_top!(l);
  }
}
