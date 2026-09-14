use core::{
  ffi::c_void,
  ptr::{eq, null},
};

use crate::{
  functions::{adjustasize::adjustasize, resize::resize},
  macros::{dummynode::dummynode, sizenode::sizenode},
  records::lua_node::LuaNode,
  type_aliases::{lua_state::lua_State, lua_table::LuaTable},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_h_resizearray(l: *mut lua_State, t: *mut LuaTable, nasize: i32) {
  unsafe {
    let nsize = if eq((*t).node as *const LuaNode, dummynode) {
      0
    } else {
      sizenode!(t)
    };

    let asize = adjustasize(t, nasize, null());

    resize(l, t, asize, nsize);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaH_resizearray")]
pub unsafe extern "C-unwind" fn lua_h_resizearray_export(
  l: *mut lua_State,
  t: *mut c_void,
  nasize: i32,
) {
  unsafe {
    lua_h_resizearray(l, t as *mut LuaTable, nasize);
  }
}
