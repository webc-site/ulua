use crate::{
  enums::type_constant_folding::Type,
  records::constant::{Constant, ConstantData},
};

pub(crate) fn cvector(x: f64, y: f64, z: f64, w: f64) -> Constant {
  Constant {
    r#type: Type::Vector,
    string_length: 0,
    data: ConstantData {
      value_vector: [x as f32, y as f32, z as f32, w as f32],
    },
  }
}
