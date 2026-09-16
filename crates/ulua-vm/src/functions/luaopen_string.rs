use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    createmetatable_lstrlib::createmetatable_mut, gmatch::gmatch, lua_l_register::lua_l_register,
    str_byte::str_byte, str_char::str_char, str_find::str_find, str_format::str_format,
    str_gsub::str_gsub, str_len::str_len, str_lower::str_lower, str_match::str_match,
    str_pack::str_pack, str_packsize::str_packsize, str_rep::str_rep, str_reverse::str_reverse,
    str_split::str_split, str_sub::str_sub, str_unpack::str_unpack, str_upper::str_upper,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_string(l: *mut lua_State) -> c_int {
  unsafe {
    // Faithful port of the `strlib[]` registration array in lstrlib.cpp:
    // {name, func} pairs ending in a {NULL, NULL} sentinel; lua_l_register
    // copies each into the `string` table.
    let strlib: [LuaLReg; 18] = [
      LuaLReg {
        name: c"byte".as_ptr(),
        func: Some(str_byte),
      },
      LuaLReg {
        name: c"char".as_ptr(),
        func: Some(str_char),
      },
      LuaLReg {
        name: c"find".as_ptr(),
        func: Some(str_find),
      },
      LuaLReg {
        name: c"format".as_ptr(),
        func: Some(str_format),
      },
      LuaLReg {
        name: c"gmatch".as_ptr(),
        func: Some(gmatch),
      },
      LuaLReg {
        name: c"gsub".as_ptr(),
        func: Some(str_gsub),
      },
      LuaLReg {
        name: c"len".as_ptr(),
        func: Some(str_len),
      },
      LuaLReg {
        name: c"lower".as_ptr(),
        func: Some(str_lower),
      },
      LuaLReg {
        name: c"match".as_ptr(),
        func: Some(str_match),
      },
      LuaLReg {
        name: c"rep".as_ptr(),
        func: Some(str_rep),
      },
      LuaLReg {
        name: c"reverse".as_ptr(),
        func: Some(str_reverse),
      },
      LuaLReg {
        name: c"sub".as_ptr(),
        func: Some(str_sub),
      },
      LuaLReg {
        name: c"upper".as_ptr(),
        func: Some(str_upper),
      },
      LuaLReg {
        name: c"split".as_ptr(),
        func: Some(str_split),
      },
      LuaLReg {
        name: c"pack".as_ptr(),
        func: Some(str_pack),
      },
      LuaLReg {
        name: c"packsize".as_ptr(),
        func: Some(str_packsize),
      },
      LuaLReg {
        name: c"unpack".as_ptr(),
        func: Some(str_unpack),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    lua_l_register(l, c"string".as_ptr(), strlib.as_ptr());
    createmetatable_mut(l);

    1
  }
}
