use crate::{
  enums::lua_status::LuaStatus, functions::lua_d_throw_ldo::lua_d_throw,
  macros::api_checknelems::api_checknelems, records::lua_state::LuaState,
};

// lapi.cpp — l_noret lua_error(LuaState* l) { api_checknelems(l, 1); luaD_throw(l, LUA_ERRRUN); }
/// `lua_error` 核心（cpp/VM/src/lapi.cpp:1546）。调用序契约（正确性，非内存安全）：
/// `l` 处于受保护帧，且栈上已压入 1 个待抛错误对象（`api_checknelems!(l, 1)` 校验，release 不校验）；
/// `lua_d_throw` 以 `LUA_ERRRUN` 沿保护帧 unwind，本函数返回 `!`（永不正常返回）。
pub fn lua_error(l: &mut LuaState) -> ! {
  unsafe {
    api_checknelems!(l, 1);
    lua_d_throw(l, LuaStatus::ErrRun as i32)
  }
}
