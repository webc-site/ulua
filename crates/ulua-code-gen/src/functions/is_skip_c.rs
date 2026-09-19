use ulua_common::enums::luau_opcode::LuauOpcode;

#[inline]
pub fn is_skip_c(op: LuauOpcode) -> bool {
  matches!(op, LuauOpcode::LOP_LOADB)
}
