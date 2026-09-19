use ulua_common::enums::luau_bytecode_type::LBC_TYPE_USERDATA;

use crate::functions::is_custom_userdata_bytecode_type::is_custom_userdata_bytecode_type;

#[inline]
pub fn is_userdata_bytecode_type(ty: u8) -> bool {
  ty as u16 == LBC_TYPE_USERDATA.0 || is_custom_userdata_bytecode_type(ty)
}
