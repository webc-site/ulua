use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::is_fast_call::is_fast_call,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::records::{bc_inst::BcInst, bytecode_graph_parser::BytecodeGraphParser};

impl<'a> BytecodeGraphParser<'a> {
  /// # Safety
  ///
  /// `inst` must be a valid, aligned, non-null pointer to a `BcInst`.
  pub unsafe fn add_jump_input(&mut self, inst: *mut BcInst, target: i32) {
    let inst = unsafe { &mut *inst };
    LUAU_ASSERT!(!is_fast_call(inst.op));
    if target < 0 {
      LUAU_ASSERT!(inst.op == LuauOpcode::LOP_LOADB);
      return;
    }
    let target = target as u32;
    let it = self.block_by_pc.find(&target);
    LUAU_ASSERT!(it.is_some());
    let bc_op = *it.unwrap();
    inst.ops.push(bc_op);
  }
}
