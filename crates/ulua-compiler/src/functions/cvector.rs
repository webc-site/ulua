use crate::records::constant::Constant;

pub(crate) fn cvector(x: f64, y: f64, z: f64, w: f64) -> Constant {
  Constant::Vector([x as f32, y as f32, z as f32, w as f32])
}
