use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkany::lua_l_checkany, lua_pcallyieldable::lua_pcallyieldable,
  },
  macros::lua_multret::LUA_MULTRET,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn lua_b_pcally(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);

    lua_pcallyieldable(l, lua_gettop(l) - 1, LUA_MULTRET, 0)
  }
}
