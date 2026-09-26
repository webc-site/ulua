use crate::enums::luau_opcode::LuauOpcode;

pub fn is_skip_c(op: LuauOpcode) -> bool {
  matches!(op, LuauOpcode::LOP_LOADB)
}
