use core::ffi::c_char;

use crate::type_aliases::compile_constant::CompileConstant;

pub type LibraryMemberConstantCallback = Option<
  unsafe extern "C-unwind" fn(
    library: *const c_char,
    member: *const c_char,
    constant: *mut CompileConstant,
  ),
>;
