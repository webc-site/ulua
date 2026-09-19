use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::sccp::{Sccp, VmConstOps},
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp 侧 def→use 边由 GraphParser 的 `addUse` 建图时同步写入 `BcInst::uses`/
  /// `BcPhi::uses`，内联器经 `addUse`/`eraseUse` 增量维护；Rust 侧反向边收敛于
  /// `SccpState.op_uses`，进入传播前必须先从最终图一次性播种，否则 SSA 工作表
  /// 永远为空，visit 检测到格值变化后无法重估消费者（循环回边场景会把陈旧常量
  /// 错误折叠），与 cpp 的不动点语义不等价。
  pub fn seed_uses(&mut self) {
    for block_idx in 0..self.func.blocks.len() {
      for &phi_op in &self.func.blocks[block_idx].phis {
        LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);
        for phi_operand in self
          .func
          .phi(phi_op)
          .operator_deref()
          .ops
          .as_slice()
          .to_vec()
        {
          self.state.record_use(phi_operand, phi_op);
        }
      }

      for inst_op in self.func.blocks[block_idx].ops.iter().copied() {
        LUAU_ASSERT!(inst_op.kind == BcOpKind::Inst);
        for inst_operand in self
          .func
          .inst(inst_op)
          .operator_deref()
          .ops
          .as_slice()
          .to_vec()
        {
          self.state.record_use(inst_operand, inst_op);
        }
      }
    }
  }
}
