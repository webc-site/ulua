use core::ffi::c_char;

use crate::type_aliases::lua_compile_constant::LuaCompileConstant;

pub type LuaLibraryMemberConstantCallback = Option<
  unsafe extern "C-unwind" fn(
    library: *const c_char,
    member: *const c_char,
    constant: *mut LuaCompileConstant,
  ),
>;
