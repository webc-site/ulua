use crate::{
  enums::type_constant_folding::Type, records::constant::Constant,
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_nil(constant: CompileConstant) {
  let target = constant as *mut Constant;

  unsafe {
    (*target).r#type = Type::Nil;
  }
}
