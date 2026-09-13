use crate::{
  enums::type_constant_folding::Type, records::constant::Constant,
  type_aliases::compile_constant::CompileConstant,
};

pub fn set_compile_constant_vector(constant: CompileConstant, x: f32, y: f32, z: f32, w: f32) {
  let target = constant as *mut Constant;

  unsafe {
    (*target).r#type = Type::Vector;
    (*target).data.value_vector[0] = x;
    (*target).data.value_vector[1] = y;
    (*target).data.value_vector[2] = z;
    (*target).data.value_vector[3] = w;
  }
}
