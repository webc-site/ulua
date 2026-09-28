use core::ptr::null;

use crate::{
  functions::{
    auxwrapcont::auxwrapcont_arm, auxwrapy::auxwrapy_arm, cocreate::cocreate,
    lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且可分配（`cocreate` 建新线程、`lua_pushcclosurek` 建带 K 回调的
/// C 闭包，均可能 GC/扩栈），须在受保护帧内调用；闭包上值来自新建线程栈，回调指针为静态函数。
/// cpp `lcorolib.cpp:340`。
pub unsafe fn cowrap(l: *mut LuaState) -> i32 {
  unsafe {
    cocreate(l);
    lua_pushcclosurek(l, Some(auxwrapy_arm), null(), 1, Some(auxwrapcont_arm));
    1
  }
}

lua_lib_fn!(pub fn cowrap, cowrap_arm);
