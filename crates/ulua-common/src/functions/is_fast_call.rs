use crate::enums::luau_opcode::LuauOpcode;

/// `Luau::isFastCall`（BytecodeUtils.h）：fastcall 系列指令。
pub fn is_fast_call(op: LuauOpcode) -> bool {
  matches!(
    op,
    LuauOpcode::LOP_FASTCALL
      | LuauOpcode::LOP_FASTCALL1
      | LuauOpcode::LOP_FASTCALL2
      | LuauOpcode::LOP_FASTCALL2K
      | LuauOpcode::LOP_FASTCALL3
      | LuauOpcode::LOP_FASTPCALL
  )
}

/// camelCase 别名（历史移植代码使用）。
pub use is_fast_call as isFastCall;
