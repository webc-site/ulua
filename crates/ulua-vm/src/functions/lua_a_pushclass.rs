//! `luaA_pushclass` — push a `LuauClass*` value onto the Lua stack.
//! C++ source: `VM/src/lapi.cpp:133`

use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  macros::{api_check::api_check, api_incr_top::api_incr_top},
  records::{gc_object::GCObject, lua_state::lua_State},
  type_aliases::luau_class::LuauClass,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaA_pushclass")]
pub unsafe fn lua_a_pushclass(l: *mut lua_State, lco: *mut LuauClass) {
  unsafe {
    api_check!(l, !lco.is_null());

    let i_o = (*l).top;
    (*i_o).value.gc = lco as *mut GCObject;
    (*i_o).set_tt(LuaType::Class as c_int);

    api_incr_top!(l);
  }
}

pub use lua_a_pushclass as luaA_pushclass;
