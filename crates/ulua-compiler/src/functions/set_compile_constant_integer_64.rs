use crate::{
  records::constant::Constant,
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_integer_64(constant: CompileConstant, l: i64) {
  unsafe { *constant.cast::<Constant>() = Constant::Integer(l) };
}
