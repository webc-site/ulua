use core::ptr::null;

use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_pushcclosurek::lua_pushcclosurek,
    lua_setfield::lua_setfield,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::common::functions::{
  int_64_add::int_64_add, int_64_ctor::int_64_ctor, int_64_div::int_64_div, int_64_eq::int_64_eq,
  int_64_idiv::int_64_idiv, int_64_index::int_64_index, int_64_le::int_64_le, int_64_lt::int_64_lt,
  int_64_mod::int_64_mod, int_64_mul::int_64_mul, int_64_newindex::int_64_newindex,
  int_64_pow::int_64_pow, int_64_sub::int_64_sub, int_64_tostring::int_64_tostring,
  int_64_unm::int_64_unm,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_setup(l: *mut lua_State) {
  unsafe {
    lua_l_newmetatable(l, c"int64".as_ptr());

    let index: LuaCfunction = Some(int_64_index);
    lua_pushcclosurek(l, index, null(), 0, None);
    lua_setfield(l, -2, c"__index".as_ptr());

    let newindex: LuaCfunction = Some(int_64_newindex);
    lua_pushcclosurek(l, newindex, null(), 0, None);
    lua_setfield(l, -2, c"__newindex".as_ptr());

    let eq: LuaCfunction = Some(int_64_eq);
    lua_pushcclosurek(l, eq, null(), 0, None);
    lua_setfield(l, -2, c"__eq".as_ptr());

    let lt: LuaCfunction = Some(int_64_lt);
    lua_pushcclosurek(l, lt, null(), 0, None);
    lua_setfield(l, -2, c"__lt".as_ptr());

    let le: LuaCfunction = Some(int_64_le);
    lua_pushcclosurek(l, le, null(), 0, None);
    lua_setfield(l, -2, c"__le".as_ptr());

    let add: LuaCfunction = Some(int_64_add);
    lua_pushcclosurek(l, add, null(), 0, None);
    lua_setfield(l, -2, c"__add".as_ptr());

    let sub: LuaCfunction = Some(int_64_sub);
    lua_pushcclosurek(l, sub, null(), 0, None);
    lua_setfield(l, -2, c"__sub".as_ptr());

    let mul: LuaCfunction = Some(int_64_mul);
    lua_pushcclosurek(l, mul, null(), 0, None);
    lua_setfield(l, -2, c"__mul".as_ptr());

    let div: LuaCfunction = Some(int_64_div);
    lua_pushcclosurek(l, div, null(), 0, None);
    lua_setfield(l, -2, c"__div".as_ptr());

    let idiv: LuaCfunction = Some(int_64_idiv);
    lua_pushcclosurek(l, idiv, null(), 0, None);
    lua_setfield(l, -2, c"__idiv".as_ptr());

    let modulo: LuaCfunction = Some(int_64_mod);
    lua_pushcclosurek(l, modulo, null(), 0, None);
    lua_setfield(l, -2, c"__mod".as_ptr());

    let pow: LuaCfunction = Some(int_64_pow);
    lua_pushcclosurek(l, pow, null(), 0, None);
    lua_setfield(l, -2, c"__pow".as_ptr());

    let unm: LuaCfunction = Some(int_64_unm);
    lua_pushcclosurek(l, unm, null(), 0, None);
    lua_setfield(l, -2, c"__unm".as_ptr());

    let tostring: LuaCfunction = Some(int_64_tostring);
    lua_pushcclosurek(l, tostring, null(), 0, None);
    lua_setfield(l, -2, c"__tostring".as_ptr());

    let ctor: LuaCfunction = Some(int_64_ctor);
    lua_pushcclosurek(l, ctor, c"int64".as_ptr(), 0, None);
    lua_setglobal(l, c"int64".as_ptr());
  }
}
