use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::call_inliner::CallInliner;

impl<'a> CallInliner<'a> {
  /// cpp `dropPrepVarArgsInInlinedPath()`：内联入口块若以 PREPVARARGS 开头则摘掉它
  /// （变参已由调用方准备）。
  pub(crate) fn drop_prep_var_args_in_inlined_path(&mut self) {
    let mapped_block_op = self.map_block_op(self.target.entry_block);

    // 块首指令先快照成 BcOp，避免与随后 `inst_op` 的可变借用重叠
    let Some(first_op) = self.caller.block_op(mapped_block_op).ops.front().copied() else {
      return;
    };

    if self.caller.inst_op(first_op).op == LuauOpcode::LOP_PREPVARARGS {
      self.caller.block_op(mapped_block_op).ops.pop_front();
    }
  }
}
