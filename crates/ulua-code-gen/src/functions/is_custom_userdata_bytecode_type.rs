use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

#[inline]
pub fn is_custom_userdata_bytecode_type(ty: u8) -> bool {
  ty >= LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8
    && ty < LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8
}
