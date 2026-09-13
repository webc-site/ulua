use core::{
  ffi::c_char,
  fmt::{Debug, Formatter, Result},
  ptr::null,
};

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::type_constant_folding::Type;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Constant {
  pub(crate) r#type: Type,
  pub(crate) string_length: u32,
  pub(crate) data: ConstantData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ConstantData {
  pub(crate) value_boolean: bool,
  pub(crate) value_number: f64,
  pub(crate) value_integer64: i64,
  pub(crate) value_vector: [f32; 4],
  pub(crate) value_table: usize,
  pub(crate) value_string: *const c_char,
}

impl DenseDefault for Constant {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl Default for Constant {
  fn default() -> Self {
    Self {
      r#type: Type::Unknown,
      string_length: 0,
      data: ConstantData {
        value_string: null(),
      },
    }
  }
}

impl Default for ConstantData {
  fn default() -> Self {
    Self {
      value_string: null(),
    }
  }
}

// Manual Debug implementation for Constant because of the union field
impl Debug for Constant {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut ds = f.debug_struct("Constant");
    ds.field("type", &self.r#type);
    ds.field("string_length", &self.string_length);
    unsafe {
      match self.r#type {
        Type::Unknown => ds.field("data", &"Unknown"),
        Type::Nil => ds.field("data", &"Nil"),
        Type::Boolean => ds.field("value_boolean", &self.data.value_boolean),
        Type::Number => ds.field("value_number", &self.data.value_number),
        Type::Integer => ds.field("value_integer64", &self.data.value_integer64),
        Type::Vector => ds.field("value_vector", &self.data.value_vector),
        Type::Table => ds.field("value_table", &self.data.value_table),
        Type::String => ds.field("value_string", &self.data.value_string),
      };
    }
    ds.finish()
  }
}
