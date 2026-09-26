use crate::common::records::bytecode_types::{LBC_TYPE_ANY, LBC_TYPE_NUMBER, LBC_TYPE_VECTOR};

pub fn vector_access_bytecode_type(member: &str) -> u8 {
  match member {
    "Magnitude" => LBC_TYPE_NUMBER,
    "Unit" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
}
