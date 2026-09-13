use crate::{
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_e_newthread::lua_e_newthread},
  macros::{api_incr_top::api_incr_top, lua_c_check_gc::luaC_checkGC, setthvalue::setthvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_newthread(l: *mut lua_State) -> *mut lua_State {
  unsafe {
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    let l1 = lua_e_newthread(l);
    setthvalue!(l, (*l).top, l1);
    api_incr_top!(l);
    let g = (*l).global;
    if let Some(userthread) = (*g).cb.userthread {
      userthread(l, l1);
    }
    l1
  }
}
