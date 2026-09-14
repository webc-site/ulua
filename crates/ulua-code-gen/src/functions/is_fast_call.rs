use ulua_common::enums::luau_opcode::LuauOpcode;

pub fn is_fast_call(op: LuauOpcode) -> bool {
  matches!(
    op,
    LuauOpcode::LOP_FASTCALL
      | LuauOpcode::LOP_FASTCALL1
      | LuauOpcode::LOP_FASTCALL2
      | LuauOpcode::LOP_FASTCALL2K
      | LuauOpcode::LOP_FASTCALL3
  )
}
