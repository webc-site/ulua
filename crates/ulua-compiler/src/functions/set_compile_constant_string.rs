use core::ffi::c_char;

use ulua_ast::records::location::Location;

use crate::{
  records::{compile_error::CompileError, constant::Constant},
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_string(constant: CompileConstant, s: *const c_char, l: usize) {
  if l > u32::MAX as usize {
    CompileError::raise(
      &Location::default(),
      format_args!("Exceeded custom string constant length limit"),
    );
  }

  unsafe { *constant.cast::<Constant>() = Constant::string(s, l as u32) };
}
