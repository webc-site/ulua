use core::{
  ffi::c_int,
  ptr::{null, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::luaM_newgco_,
  macros::{lua_c_init::luaC_init, lua_minstack::LUA_MINSTACK, size_cclosure::size_cclosure},
  records::{closure::Closure, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_new_cclosure(
  l: *mut lua_State,
  nelems: c_int,
  e: *mut LuaTable,
) -> *mut Closure {
  unsafe {
    let c = luaM_newgco_(l, size_cclosure(nelems), (*l).activememcat) as *mut Closure;

    luaC_init!(l, c, LuaType::Function as c_int);
    (*c).is_c = 1;
    (*c).env = e;
    (*c).nupvalues = nelems as u8;
    (*c).stacksize = LUA_MINSTACK as u8;
    (*c).preload = 0;
    (*c).usage = 0;
    (*c).gclist = null_mut();
    let cc = core::ptr::addr_of_mut!((*c).inner.c);
    (*cc).f = None;
    (*cc).cont = None;
    (*cc).debugname = null();

    c
  }
}

pub use lua_f_new_cclosure as luaF_newCclosure;
