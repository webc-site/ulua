use core::{
  ffi::{c_char, c_int, c_void},
  slice::from_raw_parts,
};

use ulua_vm::{
  functions::{
    lua_objlen::lua_objlen, lua_pushinteger::lua_pushinteger, lua_pushstring::lua_pushstring,
    lua_rawseti::lua_rawseti, lua_setfield::lua_setfield,
  },
  macros::lua_newtable::lua_newtable,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_callback(
  context: *mut c_void,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  unsafe {
    let l = context as *mut lua_State;

    lua_newtable(l);

    lua_pushstring(l, function);
    lua_setfield(l, -2, c"name".as_ptr());

    lua_pushinteger(l, linedefined);
    lua_setfield(l, -2, c"linedefined".as_ptr());

    lua_pushinteger(l, depth);
    lua_setfield(l, -2, c"depth".as_ptr());

    // 切片视图跳过未命中的槽位，避免逐指针累加
    let hits = from_raw_parts(hits, size);
    for (i, &hit) in hits.iter().enumerate().filter(|&(_, &hit)| hit != -1) {
      lua_pushinteger(l, hit);
      lua_rawseti(l, -2, i as c_int);
    }

    lua_rawseti(l, -2, lua_objlen(l, -2) + 1);
  }
}
