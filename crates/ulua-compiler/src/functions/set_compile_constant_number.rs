use crate::{records::constant::Constant, type_aliases::compile_constant::CompileConstant};

pub fn set_compile_constant_number(constant: CompileConstant, n: f64) {
  unsafe { *constant.cast::<Constant>() = Constant::Number(n) };
}
