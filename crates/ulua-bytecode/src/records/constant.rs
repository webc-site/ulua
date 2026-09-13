use core::fmt::{self, Debug, Formatter};

use crate::enums::r#type::Type;

#[derive(Debug, Clone, Copy)]
pub struct Constant {
  pub(crate) r#type: Type,
  pub(crate) value: ConstantValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ConstantValue {
  pub(crate) value_boolean: bool,
  pub(crate) value_number: f64,
  pub(crate) value_integer64: i64,
  pub(crate) value_vector: [f32; 4],
  pub(crate) value_string: u32,
  pub(crate) value_import: u32,
  pub(crate) value_table: u32,
  pub(crate) value_closure: u32,
  pub(crate) value_class_shape: u32,
}

// A union has no active-variant tag of its own, so `Debug` cannot read a field
// safely; print it opaquely. The active variant is known only via the owning
// `Constant`'s `type` discriminant.
impl Debug for ConstantValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("ConstantValue(..)")
  }
}
