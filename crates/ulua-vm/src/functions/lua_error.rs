use crate::{
  enums::lua_status::LuaStatus, functions::lua_d_throw_ldo::lua_d_throw,
  macros::api_checknelems::api_checknelems, type_aliases::lua_state::lua_State,
};

// lapi.cpp — l_noret lua_error(lua_State* l) { api_checknelems(l, 1); luaD_throw(l, LUA_ERRRUN); }
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_error(l: *mut lua_State) -> ! {
  unsafe {
    api_checknelems!(l, 1);
    lua_d_throw(l, LuaStatus::ErrRun as i32)
  }
}
