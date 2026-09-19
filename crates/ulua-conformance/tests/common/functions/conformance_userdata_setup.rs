use core::{ffi::CStr, ptr::null};

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

/// 在栈顶元表上登记元方法：压入 C 函数并写入 `name` 字段。
unsafe fn set_meta_method(l: *mut lua_State, name: &CStr, f: LuaCfunction) {
  unsafe {
    lua_pushcclosurek(l, f, null(), 0, None);
    lua_setfield(l, -2, name.as_ptr());
  }
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_setup(l: *mut lua_State) {
  unsafe {
    lua_l_newmetatable(l, c"int64".as_ptr());

    set_meta_method(l, c"__index", Some(int_64_index));
    set_meta_method(l, c"__newindex", Some(int_64_newindex));
    set_meta_method(l, c"__eq", Some(int_64_eq));
    set_meta_method(l, c"__lt", Some(int_64_lt));
    set_meta_method(l, c"__le", Some(int_64_le));
    set_meta_method(l, c"__add", Some(int_64_add));
    set_meta_method(l, c"__sub", Some(int_64_sub));
    set_meta_method(l, c"__mul", Some(int_64_mul));
    set_meta_method(l, c"__div", Some(int_64_div));
    set_meta_method(l, c"__idiv", Some(int_64_idiv));
    set_meta_method(l, c"__mod", Some(int_64_mod));
    set_meta_method(l, c"__pow", Some(int_64_pow));
    set_meta_method(l, c"__unm", Some(int_64_unm));
    set_meta_method(l, c"__tostring", Some(int_64_tostring));

    // 构造函数注册为全局 `int64`
    lua_pushcclosurek(l, Some(int_64_ctor), c"int64".as_ptr(), 0, None);
    lua_setglobal(l, c"int64".as_ptr());
  }
}
