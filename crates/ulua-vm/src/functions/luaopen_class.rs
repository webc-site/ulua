use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    class_classof::class_classof, class_isinstance::class_isinstance,
    lua_l_register::lua_l_register,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_class(l: *mut lua_State) -> c_int {
  unsafe {
    let class_lib: [LuaLReg; 3] = [
      LuaLReg {
        name: c"isinstance".as_ptr(),
        func: Some(class_isinstance),
      },
      LuaLReg {
        name: c"classof".as_ptr(),
        func: Some(class_classof),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    lua_l_register(l, c"class".as_ptr(), class_lib.as_ptr());
    1
  }
}
