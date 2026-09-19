use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{Constness, Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `Sccp::visitPhi`：合并各操作数格值，变化时把使用点投入 SSA 工作表。
  pub fn visit_phi(&mut self, phi_op: BcOp) {
    LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);

    // 借用拆分：phi 操作数只读借 `self.func`，`operand_lattice` 写 `self.state`，
    // 两者是 Sccp 的不相交字段，无需快照整份操作数表。
    let mut fold = Constness::Undetermined;
    {
      let phi_ops = self.func.phi(phi_op).operator_deref().ops.as_slice();
      LUAU_ASSERT!(!phi_ops.is_empty());
      for &op in phi_ops {
        fold = self.state.operand_lattice(op).merge(&fold);
      }
    }

    let prev_lattice = *self.state.op_constness.get_or_insert(phi_op);
    if fold != prev_lattice {
      self.state.defer_uses_to_ssa(phi_op);
    }
    self.state.op_constness.insert(phi_op, fold);
  }

  /// cpp `Sccp::visitInst`：求值、对比旧格值、登记跳转落点。
  pub fn visit_inst(&mut self, inst_op: BcOp) {
    LUAU_ASSERT!(inst_op.kind == BcOpKind::Inst);

    // 借用拆分：本块只读借 `self.func`，跨后续 `&mut self` 调用（evaluate 等）
    // 仅携带 Copy 的标量与 BcOp，不持有任何切片，免整份 ops 快照。
    let (opcode, inst_block, capture_ref_src) = {
      let inst_repr = self.func.inst(inst_op).operator_deref();
      let opcode = inst_repr.op;
      let inst_block = inst_repr.block;

      // CAPTURE REF 的源可经 SETUPVAL 外部修改，SSA 图不建模该别名，
      // 将源标记为非常量以避免折叠出陈旧值
      let mut capture_ref_src = None;
      if opcode == LuauOpcode::LOP_CAPTURE && inst_repr.ops.len() >= 2 {
        let capture_type_op = inst_repr.ops[0];
        LUAU_ASSERT!(capture_type_op.kind == BcOpKind::Imm);
        let capture_imm = *self.func.imm_op(capture_type_op);
        let is_ref = capture_imm.kind == BcImmKind::Int
          // Safety: kind == Int 时 value_int 为活跃字段
          && unsafe { capture_imm.value.value_int } == (LuauCaptureType::LCT_REF as u32) as i32;
        if is_ref {
          capture_ref_src = Some(inst_repr.ops[1]);
        }
      }
      (opcode, inst_block, capture_ref_src)
    };

    if let Some(src_op) = capture_ref_src {
      let prev = *self.state.op_constness.get_or_insert(src_op);
      if !matches!(prev, Constness::NotAConstant)
        && matches!(src_op.kind, BcOpKind::Inst | BcOpKind::Phi)
      {
        self
          .state
          .op_constness
          .insert(src_op, Constness::NotAConstant);
        self.state.defer_uses_to_ssa(src_op);
      }
    }

    let lattice = self.evaluate(opcode, inst_op);
    let prev_lattice = *self.state.op_constness.get_or_insert(inst_op);

    let new_val = lattice.merge(&prev_lattice);
    if new_val != prev_lattice {
      self.state.defer_uses_to_ssa(inst_op);
    }

    for target in self.jump_targets(inst_op).as_slice() {
      LUAU_ASSERT!(target.block_op.kind == BcOpKind::Block);
      let block_idx = target.block_op.index;
      if !target.dead {
        self
          .state
          .block_uses
          .get_or_insert(block_idx)
          .insert(inst_block);
        if !self.state.flow_worklist_set.contains(&target.block_op) {
          self.state.flow_worklist.push_back(target.block_op);
        }
      }
    }

    self.state.op_constness.insert(inst_op, new_val);
  }

  /// cpp `Sccp::propagate`：flow 工作表驱动块遍历，SSA 工作表驱动格值收敛。
  pub fn propagate(&mut self) {
    let entry_op = self.func.entry_block;
    let exit_op = self.func.exit_block;
    // entry/exit 块恒为活块（exit 因序列化要求）
    self
      .state
      .block_uses
      .get_or_insert(entry_op.index)
      .insert(entry_op);
    self
      .state
      .block_uses
      .get_or_insert(exit_op.index)
      .insert(exit_op);

    self.state.flow_worklist.push_back(entry_op);

    while !self.state.flow_worklist.is_empty() || !self.state.ssa_worklist.is_empty() {
      while let Some(block_op) = self.state.flow_worklist.pop_front() {
        if self.state.flow_worklist_set.contains(&block_op) {
          continue;
        }

        // 拷贝到复用 scratch（只拷贝不分配），借用随即结束，后续 visit 可再借 &mut self。
        {
          let block_ref = self.func.block(block_op);
          let block = block_ref.operator_deref();
          let scratch = &mut self.scratch;
          scratch.phis.clear();
          scratch.phis.extend_from_slice(block.phis.as_slice());
          scratch.ops.clear();
          scratch.ops.extend(block.ops.iter().copied());
          scratch.successors.clear();
          scratch
            .successors
            .extend_from_slice(block.successors.as_slice());
        }

        for i in 0..self.scratch.phis.len() {
          let phi_op = self.scratch.phis[i];
          LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);
          self.visit_phi(phi_op);
        }

        for i in 0..self.scratch.ops.len() {
          let op = self.scratch.ops[i];
          LUAU_ASSERT!(op.kind == BcOpKind::Inst);
          self.visit_inst(op);
        }

        let block_ends_with_branch = self
          .scratch
          .ops
          .last()
          .copied()
          .is_some_and(|last| last.kind == BcOpKind::Inst && !self.jump_targets(last).is_empty());

        for i in 0..self.scratch.successors.len() {
          let succ_edge = self.scratch.successors[i];
          if succ_edge.kind == BcBlockEdgeKind::Fallthrough {
            let succ_idx = succ_edge.target.index;
            // 分支已覆盖该后继时跳过，避免重复入队
            if block_ends_with_branch
              && !self
                .state
                .block_uses
                .get_or_insert(succ_idx)
                .contains(&block_op)
            {
              continue;
            }
            self
              .state
              .block_uses
              .get_or_insert(succ_idx)
              .insert(block_op);
            if !self.state.flow_worklist_set.contains(&succ_edge.target) {
              self.state.flow_worklist.push_back(succ_edge.target);
            }
          }
        }

        self.state.flow_worklist_set.insert(block_op);
      }

      while let Some(op) = self.state.ssa_worklist.pop_front() {
        match op.kind {
          BcOpKind::Inst => self.visit_inst(op),
          BcOpKind::Phi => self.visit_phi(op),
          _ => {}
        }
      }
    }
  }
}
