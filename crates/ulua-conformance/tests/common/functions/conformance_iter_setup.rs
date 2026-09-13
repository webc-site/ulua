use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
  type_aliases::{lua_c_function::LuaCfunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  c_yielding_iterator::c_yielding_iterator,
  c_yielding_iterator_continuation::c_yielding_iterator_continuation,
  setup_native_helpers::setup_native_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_iter_setup(l: *mut lua_State) {
  unsafe {
    setup_native_helpers(l);

    let iterator: LuaCfunction = Some(c_yielding_iterator);
    let continuation: LuaContinuation = Some(c_yielding_iterator_continuation);

    lua_pushcclosurek(l, iterator, c"cYieldingIterator".as_ptr(), 0, continuation);
    lua_setglobal(l, c"cYieldingIterator".as_ptr());
  }
}
