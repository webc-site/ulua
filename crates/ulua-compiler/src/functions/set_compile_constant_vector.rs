use crate::{records::constant::Constant, type_aliases::compile_constant::CompileConstant};

pub fn set_compile_constant_vector(constant: CompileConstant, x: f32, y: f32, z: f32, w: f32) {
  unsafe { *constant.cast::<Constant>() = Constant::Vector([x, y, z, w]) };
}
