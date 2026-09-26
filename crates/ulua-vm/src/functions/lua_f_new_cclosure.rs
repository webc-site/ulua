use core::ptr::{null, null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::lua_m_newgco,
  macros::{lua_c_init::luaC_init, lua_minstack::LUA_MINSTACK, size_cclosure::size_cclosure},
  records::{closure::Closure, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须指向存活 `LuaState` 且分配器/GC 记账可用（cpp lfunc.cpp:83）：闭包按
/// `size_cclosure(nelems)` 分配，`nelems` 即实际 upvalue 槽数；`e` 仅按指针存入 env、不解引用。
pub unsafe fn lua_f_new_cclosure(l: *mut LuaState, nelems: i32, e: *mut LuaTable) -> *mut Closure {
  // Safety: 契约保证 `l` 存活可用分配器，块内按 size_cclosure(nelems) 分配并仅写本闭包自有字段
  unsafe {
    let c = lua_m_newgco(l, size_cclosure(nelems), (*l).activememcat) as *mut Closure;

    luaC_init!(l, c, LuaType::Function as i32);
    let cl = &mut *c;
    cl.is_c = 1;
    cl.env = e;
    cl.nupvalues = nelems as u8;
    cl.stacksize = LUA_MINSTACK as u8;
    cl.preload = 0;
    cl.gclist = null_mut();

    let cc = &mut cl.inner.c;
    cc.f = None;
    cc.cont = None;
    cc.debugname = null();

    c
  }
}
