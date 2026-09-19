use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::is_fast_call::is_fast_call,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addJumpInput(BcRef<BcInst>, int32_t)`：把跳转目标 PC 解析成块操作数。
  pub fn add_jump_input(&mut self, inst: BcOp, target: i32) {
    let inst_op = self.func.inst(inst).operator_deref().op;
    LUAU_ASSERT!(!is_fast_call(inst_op));
    if target < 0 {
      LUAU_ASSERT!(inst_op == LuauOpcode::LOP_LOADB);
      return;
    }
    let target = target as u32;
    let it = self.block_by_pc.find(&target);
    LUAU_ASSERT!(it.is_some());
    let bc_op = *it.unwrap();
    self.func.inst_op(inst).ops.push(bc_op);
  }
}
