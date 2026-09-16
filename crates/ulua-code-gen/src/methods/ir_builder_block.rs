use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_builder::IrBuilder, ir_op::IrOp, label::Label},
};

impl IrBuilder {
  pub fn block(&mut self, mut kind: IrBlockKind) -> IrOp {
    CODEGEN_ASSERT!(kind != IrBlockKind::Fallback);

    if kind == IrBlockKind::Internal && self.active_fastcall_fallback {
      kind = IrBlockKind::Fallback;
    }

    let index = self.function.blocks.len() as u32;
    self.function.blocks.push(IrBlock {
      kind,
      flags: 0,
      use_count: 0,
      start: !0u32,
      finish: !0u32,
      sortkey: 0,
      chainkey: 0,
      expected_next_block: !0u32,
      startpc: !0u32,
      label: Label::default(),
    });
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, index)
  }
}
