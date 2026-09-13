use ulua_common::enums::luau_bytecode_type::{
  LBC_TYPE_TAGGED_USERDATA_BASE, LBC_TYPE_TAGGED_USERDATA_END,
};

#[inline]
pub fn is_custom_userdata_bytecode_type(ty: u8) -> bool {
  ty >= LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8 && ty < LBC_TYPE_TAGGED_USERDATA_END.0 as u8
}
