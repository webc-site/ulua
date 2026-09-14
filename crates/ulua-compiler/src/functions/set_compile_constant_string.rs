use core::ffi::c_char;

use ulua_ast::records::location::Location;

use crate::{
  enums::type_constant_folding::Type,
  records::{compile_error::CompileError, constant::Constant},
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_string(constant: CompileConstant, s: *const c_char, l: usize) {
  let target = constant as *mut Constant;

  if l > u32::MAX as usize {
    CompileError::raise(
      &Location::default(),
      format_args!("Exceeded custom string constant length limit"),
    );
  }

  unsafe {
    (*target).r#type = Type::String;
    (*target).string_length = l as u32;
    (*target).data.value_string = s;
  }
}
