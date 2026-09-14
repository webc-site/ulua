use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

use crate::{
  functions::{index_2_addr::index_2_addr, lua_h_clone::lua_h_clone},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, hvalue::hvalue, sethvalue::sethvalue,
    ttistable::ttistable,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_clonetable(l: *mut lua_State, idx: c_int) {
  unsafe {
    // The C++ source:
    // StkId t = index2addr(l, idx);
    // api_check(l, ttistable(t));
    // LuaTable* tt = luaH_clone(l, hvalue(t));
    // sethvalue(l, l->top, tt);
    // api_incr_top(l);

    // We must cast the stubbed function pointers to their real signatures to call them.
    // Rust does not allow direct casting from fn item to fn pointer with different signature,
    // so we cast through a usize.
    let index2addr_fn: unsafe fn(*mut lua_State, c_int) -> StkId =
      transmute(index_2_addr as *const c_void);
    let t: StkId = index2addr_fn(l, idx);

    api_check!(l, ttistable!(t));

    let lua_h_clone_fn: unsafe fn(*mut lua_State, *mut LuaTable) -> *mut LuaTable =
      transmute(lua_h_clone as *const c_void);
    let tt: *mut LuaTable = lua_h_clone_fn(l, hvalue!(t));

    sethvalue!(l, (*l).top, tt);
    api_incr_top!(l);
  }
}
