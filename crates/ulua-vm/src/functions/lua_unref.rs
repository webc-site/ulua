use core::ffi::c_int;

use crate::{
  functions::lua_h_getnum::lua_h_getnum,
  macros::{
    api_check::api_check, hvalue::hvalue, lua_o_nilobject::luaO_nilobject, lua_refnil::LUA_REFNIL,
    registry::registry, setnvalue::setnvalue,
  },
  records::{global_state::global_State, lua_state::lua_State},
  type_aliases::{lua_table::LuaTable, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_unref(l: *mut lua_State, ref_: c_int) {
  unsafe {
    if ref_ <= LUA_REFNIL {
      return;
    }

    let g: *mut global_State = (*l).global;

    // The hvalue! macro expects a pointer to a TValue.
    // registry!(l) returns &(*(*l).global).registry, which is a &TValue.
    let reg_tvalue_ptr: *const TValue = registry!(l);
    let reg: *mut LuaTable = hvalue!(reg_tvalue_ptr) as *const _ as *mut LuaTable;

    let slot: *const TValue = lua_h_getnum(reg, ref_);

    api_check!(l, slot != luaO_nilobject);

    // similar to how 'luaH_setnum' makes non-nil slot value mutable
    let mutable_slot = slot as *mut TValue;

    // NB: no barrier needed because value isn't collectable (it's a number)
    setnvalue!(mutable_slot, (*g).registryfree as f64);

    (*g).registryfree = ref_;
  }
}
