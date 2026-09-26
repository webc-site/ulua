use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_rawcheckstack::lua_rawcheckstack, lua_xmove::lua_xmove,
  },
  macros::{co_status_error::CO_STATUS_ERROR, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn auxresumecont(l: *mut LuaState, co: *mut LuaState) -> i32 {
  unsafe {
    if (*co).status == LuaStatus::Ok as u8 || (*co).status == LuaStatus::Yield as u8 {
      let nres = (*co).top.offset_from((*co).base) as i32;
      if lua_checkstack(l, nres + 1) == 0 {
        luaL_error!(l, "too many results to resume");
      }
      lua_xmove(co, l, nres);
      nres
    } else {
      lua_rawcheckstack(l, 2);
      lua_xmove(co, l, 1);
      CO_STATUS_ERROR
    }
  }
}
