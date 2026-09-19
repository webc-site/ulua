const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;

pub fn userdata_index_to_type(userdata_index: u8) -> u8 {
  LBC_TYPE_TAGGED_USERDATA_BASE + userdata_index
}
