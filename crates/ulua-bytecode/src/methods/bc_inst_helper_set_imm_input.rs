use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::bc_inst_helper::BcInstHelper,
};

impl BcInstHelper<'_> {
  pub fn set_imm_input(&mut self, input_idx: u32, value: i32) {
    if self.get_bc_op(input_idx).kind == BcOpKind::None {
      let imm_op = self.graph.add_imm(BcImmKind::Int);
      self.set_bc_op(input_idx, imm_op);
    }

    let op = self.get_bc_op(input_idx);
    let imm = self.graph.imm_op(op);

    LUAU_ASSERT!(imm.kind == BcImmKind::Int);
    imm.value.value_int = value;
  }
}
