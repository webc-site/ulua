use crate::{
  enums::type_constant_folding::Type, records::constant::Constant,
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_boolean(constant: CompileConstant, b: bool) {
  let target = constant as *mut Constant;

  unsafe {
    (*target).r#type = Type::Boolean;
    (*target).data.value_boolean = b;
  }
}
