use core::ffi::c_int;

use ulua_vm::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

use crate::common::functions::{
  conformance_tables_make_lud::conformance_tables_make_lud,
  push_cfunction_global::push_cfunction_global,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_setup(l: *mut LuaState) {
  let make_lud: LuaCFunction =
    Some(conformance_tables_make_lud as unsafe extern "C-unwind" fn(*mut LuaState) -> c_int);
  push_cfunction_global(l, make_lud, b"makelud\0");
}
