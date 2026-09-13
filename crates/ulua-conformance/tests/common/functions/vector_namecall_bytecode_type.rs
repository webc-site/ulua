const LBC_TYPE_ANY: u8 = 15;
const LBC_TYPE_NUMBER: u8 = 2;
const LBC_TYPE_VECTOR: u8 = 8;

pub fn vector_namecall_bytecode_type(member: &str) -> u8 {
  match member {
    "Dot" => LBC_TYPE_NUMBER,
    "Cross" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
}
