const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;

pub fn type_to_userdata_index(r#type: u8) -> u8 {
  // Underflow will push the type into a value that is not comparable to any kUserdata* constants
  r#type.wrapping_sub(LBC_TYPE_TAGGED_USERDATA_BASE)
}
