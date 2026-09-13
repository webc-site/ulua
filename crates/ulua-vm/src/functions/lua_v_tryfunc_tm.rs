use crate::{
  enums::tms::TMS,
  functions::lua_t_gettmbyobj::lua_t_gettmbyobj,
  macros::{lua_g_typeerror::luaG_typeerror, setobj_2_s::setobj2s, ttisfunction::ttisfunction},
  type_aliases::{lua_state::LuaState, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_v_tryfunc_tm")]
pub unsafe fn lua_v_tryfunc_tm(l: *mut LuaState, func: StkId) {
  unsafe {
    let tm = lua_t_gettmbyobj(l, func, TMS::TmCall);
    if !ttisfunction!(tm) {
      luaG_typeerror!(l, func, c"call".as_ptr());
    }

    let mut p = (*l).top;
    while p > func {
      setobj2s!(l, p, p.wrapping_sub(1));
      p = p.wrapping_sub(1);
    }

    (*l).top = (*l).top.wrapping_add(1);
    setobj2s!(l, func, tm);
  }
}
