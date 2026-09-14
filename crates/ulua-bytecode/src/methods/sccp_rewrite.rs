use alloc::vec::Vec;

use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::is_jump_d::is_jump_d, macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_block_edge::BcBlockEdge,
    bc_op::BcOp,
    sccp::{Constness, ConstnessLattice, Sccp, VmConstOps},
  },
  type_aliases::bc_edges::BcEdges,
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `Sccp::rewrite`：常量传播完成后的图重写四部曲。
  pub fn rewrite(&mut self) {
    self.arith_to_k();
    self.replace_uses();
    self.simplify_phis();
    self.update_block_uses();
  }

  /// cpp `Sccp::isLoadInst`：已是载入指令者不再改写。
  fn is_load_inst(&self, op: BcOp) -> bool {
    if op.kind != BcOpKind::Inst {
      return false;
    }
    matches!(
      self.func.inst(op).operator_deref().op,
      LuauOpcode::LOP_LOADK
        | LuauOpcode::LOP_LOADKX
        | LuauOpcode::LOP_LOADN
        | LuauOpcode::LOP_LOADB
        | LuauOpcode::LOP_LOADNIL
    )
  }

  /// cpp `Sccp::replaceUses`：常量指令改写为载入或删除死跳转。
  fn replace_uses(&mut self) {
    let entries: Vec<(BcOp, ConstnessLattice)> = self
      .state
      .op_constness
      .iter()
      .map(|(op, lattice)| (*op, *lattice))
      .collect();
    for (op, lattice) in entries {
      if !matches!(lattice.kind, Constness::ImmConstant | Constness::VmConstant) {
        continue;
      }
      if op.kind != BcOpKind::Inst {
        continue;
      }
      if self.is_load_inst(op) {
        continue;
      }

      let opcode = self.func.inst(op).operator_deref().op;
      LUAU_ASSERT!(opcode != LuauOpcode::LOP_JUMPX);

      // JUMPX 不被 GraphParser 解析，is_jump_d 在此安全
      if is_jump_d(opcode) {
        self.remove_dead_edges(op);
        self.erase_op(op);
      } else {
        self.rewrite_to_load(op, &lattice);
      }
    }
  }

  /// cpp `BcFunction::eraseOp`：从所属块移除指令，并解除其作为消费者的全部反向边。
  fn erase_op(&mut self, op: BcOp) {
    let (block, old_ops) = {
      let inst = self.func.inst(op);
      (
        inst.operator_deref().block,
        inst.operator_deref().ops.as_slice().to_vec(),
      )
    };
    for used in old_ops {
      self.state.erase_use(op, used);
    }
    self.func.block_op(block).ops.retain(|x| *x != op);
  }

  /// cpp `Sccp::rewriteToLoad`：原位改写为载入指令，op 身份不变，既有使用点全部有效。
  fn rewrite_to_load(&mut self, op: BcOp, lattice: &ConstnessLattice) {
    let old_ops: Vec<BcOp> = self.func.inst(op).operator_deref().ops.as_slice().to_vec();
    for used in old_ops {
      self.state.erase_use(op, used);
    }

    if lattice.kind == Constness::VmConstant {
      let const_op = lattice.vm_const.unwrap();
      let inst = self.func.inst_op(op);
      inst.op = LuauOpcode::LOP_LOADK;
      inst.ops.clear();
      inst.ops.push_back(const_op);
    } else {
      LUAU_ASSERT!(lattice.kind == Constness::ImmConstant);
      let imm = lattice.imm_const.unwrap();
      let imm_op = self.func.add_imm_alt_b(imm);
      let inst = self.func.inst_op(op);
      inst.op = if imm.kind == BcImmKind::Boolean {
        LuauOpcode::LOP_LOADB
      } else {
        LuauOpcode::LOP_LOADN
      };
      inst.ops.clear();
      inst.ops.push_back(imm_op);
    }
  }

  /// cpp `Sccp::removeDeadEdges`：剔除死边，保证活目标持有 fallthrough 边。
  fn remove_dead_edges(&mut self, op: BcOp) {
    let targets = self.jump_targets(op);
    let block_op = self.func.inst(op).operator_deref().block;

    let mut live_target = BcOp::new();
    let mut has_live = false;

    for target in targets.as_slice() {
      if target.dead {
        let kept: BcEdges = self
          .func
          .block(block_op)
          .operator_deref()
          .successors
          .as_slice()
          .iter()
          .cloned()
          .filter(|edge| edge.target != target.block_op)
          .collect();
        self.func.block_op(block_op).successors = kept;
      } else {
        live_target = target.block_op;
        has_live = true;
      }
    }

    if !has_live {
      return;
    }

    // 活目标必须有 fallthrough 边
    let block = self.func.block_op(block_op);
    let mut has_fallthrough = false;
    for edge in block.successors.as_mut_slice() {
      if edge.target == live_target {
        edge.kind = BcBlockEdgeKind::Fallthrough;
        has_fallthrough = true;
        break;
      }
    }

    if !has_fallthrough {
      block.successors.push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: live_target,
      });
    }
  }
}
