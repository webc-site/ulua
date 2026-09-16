use core::ffi::c_void;

use crate::{
  functions::set_compile_constant_number::set_compile_constant_number,
  type_aliases::lua_compile_constant::LuaCompileConstant,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luau_set_compile_constant_number(
  constant: *mut LuaCompileConstant,
  n: f64,
) {
  set_compile_constant_number(constant as *mut c_void, n);
}
