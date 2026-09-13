const LBC_TYPE_NUMBER: u8 = 2;
const LBC_TYPE_VECTOR: u8 = 8;
const LBC_TYPE_ANY: u8 = 15;

pub fn vector_access_bytecode_type(member: &str) -> u8 {
  match member {
    "Magnitude" => LBC_TYPE_NUMBER,
    "Unit" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
}
