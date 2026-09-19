use crate::{
  records::constant::Constant,
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_nil(constant: CompileConstant) {
  unsafe { *constant.cast::<Constant>() = Constant::Nil };
}
