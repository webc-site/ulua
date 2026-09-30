use core::ptr::null;

use crate::{
  functions::iter_aux::iter_aux_arm, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`check_bytes(1)` 要求索引 1 为字符串否则抛错回退；随后
/// `push_c_function`(C 闭包)/`push_value(1)`/`push_integer` 连压 3 个值，`(*l).top` 后须留 ≥3 空槽
/// （由 utf8 库调用点保证）；push 可触发 GC。
/// cpp VM/src/lutf8lib.cpp:267
pub unsafe fn iter_codes(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_bytes(1);
    (*l).push_c_function(Some(iter_aux_arm), null());
    (*l).push_value(1);
    (*l).push_integer(0);
    3
  }
}

lua_lib_fn!(pub fn iter_codes, iter_codes_arm);
