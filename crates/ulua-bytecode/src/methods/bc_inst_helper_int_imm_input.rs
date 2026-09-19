use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bc_inst_helper::BcInstHelper;

impl BcInstHelper<'_> {
  pub fn int_imm_input(&mut self, input_idx: u32) -> i32 {
    let inst = self.operator_deref();
    LUAU_ASSERT!((input_idx as usize) < inst.ops.len());

    let op = inst.ops[input_idx as usize];
    let imm = self.graph.imm_op(op);

    // SAFETY：图构建阶段该操作数只可能由 `addImmInput(int32_t)` 写入（BcImmKind::Int）。
    unsafe { imm.value.value_int }
  }
}
