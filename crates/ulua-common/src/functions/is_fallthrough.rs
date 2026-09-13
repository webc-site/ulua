use crate::enums::luau_opcode::LuauOpcode;

/// 非终结指令（不含 RETURN/JUMP/JUMPBACK/JUMPX）。
pub fn is_fallthrough(op: LuauOpcode) -> bool {
  !matches!(
    op,
    LuauOpcode::LOP_RETURN
      | LuauOpcode::LOP_JUMP
      | LuauOpcode::LOP_JUMPBACK
      | LuauOpcode::LOP_JUMPX
  )
}
