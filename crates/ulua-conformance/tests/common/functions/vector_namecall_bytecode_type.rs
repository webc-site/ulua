use crate::common::records::bytecode_types::{LBC_TYPE_ANY, LBC_TYPE_NUMBER, LBC_TYPE_VECTOR};

pub fn vector_namecall_bytecode_type(member: &str) -> u8 {
  match member {
    "Dot" => LBC_TYPE_NUMBER,
    "Cross" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
}
