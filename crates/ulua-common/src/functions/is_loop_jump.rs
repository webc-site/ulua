use crate::enums::luau_opcode::LuauOpcode;

/// 循环跳转指令（JUMPBACK/FORGLOOP/FORNLOOP）。
pub fn is_loop_jump(op: LuauOpcode) -> bool {
  matches!(
    op,
    LuauOpcode::LOP_JUMPBACK | LuauOpcode::LOP_FORGLOOP | LuauOpcode::LOP_FORNLOOP
  )
}
