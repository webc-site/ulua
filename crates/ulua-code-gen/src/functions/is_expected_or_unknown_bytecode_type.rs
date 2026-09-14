use ulua_common::enums::luau_bytecode_type::{LBC_TYPE_ANY, LuauBytecodeType};

#[inline]
pub fn is_expected_or_unknown_bytecode_type(ty: u8, expected: LuauBytecodeType) -> bool {
  let ty_u16 = ty as u16;
  ty_u16 == LBC_TYPE_ANY.0 || ty_u16 == expected.0
}
