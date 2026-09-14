use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    b_and::b_and, b_arshift::b_arshift, b_countlz::b_countlz, b_countrz::b_countrz,
    b_extract::b_extract, b_lrot::b_lrot, b_lshift::b_lshift, b_not::b_not, b_or::b_or,
    b_replace::b_replace, b_rrot::b_rrot, b_rshift::b_rshift, b_swap::b_swap, b_test::b_test,
    b_xor::b_xor, lua_l_register::lua_l_register,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_bit32(l: *mut lua_State) -> c_int {
  unsafe {
    // Faithful port of bitlib[] in lbitlib.cpp (Lua name -> b_* function).
    let bitlib: [LuaLReg; 16] = [
      LuaLReg {
        name: c"arshift".as_ptr(),
        func: Some(b_arshift),
      },
      LuaLReg {
        name: c"band".as_ptr(),
        func: Some(b_and),
      },
      LuaLReg {
        name: c"bnot".as_ptr(),
        func: Some(b_not),
      },
      LuaLReg {
        name: c"bor".as_ptr(),
        func: Some(b_or),
      },
      LuaLReg {
        name: c"bxor".as_ptr(),
        func: Some(b_xor),
      },
      LuaLReg {
        name: c"btest".as_ptr(),
        func: Some(b_test),
      },
      LuaLReg {
        name: c"extract".as_ptr(),
        func: Some(b_extract),
      },
      LuaLReg {
        name: c"lrotate".as_ptr(),
        func: Some(b_lrot),
      },
      LuaLReg {
        name: c"lshift".as_ptr(),
        func: Some(b_lshift),
      },
      LuaLReg {
        name: c"replace".as_ptr(),
        func: Some(b_replace),
      },
      LuaLReg {
        name: c"rrotate".as_ptr(),
        func: Some(b_rrot),
      },
      LuaLReg {
        name: c"rshift".as_ptr(),
        func: Some(b_rshift),
      },
      LuaLReg {
        name: c"countlz".as_ptr(),
        func: Some(b_countlz),
      },
      LuaLReg {
        name: c"countrz".as_ptr(),
        func: Some(b_countrz),
      },
      LuaLReg {
        name: c"byteswap".as_ptr(),
        func: Some(b_swap),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    lua_l_register(l, c"bit32".as_ptr(), bitlib.as_ptr());

    1
  }
}
