use ulua_common::{
  enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT,
  records::small_vector::SmallVector,
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{ConditionState, ConstnessLattice, JumpTarget, Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `Sccp::makeBoolImm`
  pub fn make_bool_imm(&self, value: bool) -> ConstnessLattice {
    ConstnessLattice::imm_constant(self.vm_ops.make_imm_bool(value))
  }

  /// cpp `Sccp::getFallthrough`：取块的唯一 fallthrough 后继；多个则违反 CFG 不变量。
  pub fn get_fallthrough(&self, block_op: BcOp) -> Option<BcOp> {
    let mut fallthrough = None;
    for succ in self
      .func
      .block(block_op)
      .operator_deref()
      .successors
      .as_slice()
    {
      if succ.kind == BcBlockEdgeKind::Fallthrough {
        if fallthrough.is_some() {
          LUAU_ASSERT!(false, "Multiple fallthroughs");
          return None;
        }
        fallthrough = Some(succ.target);
      }
    }
    fallthrough
  }

  /// cpp `Sccp::conditionalTargets`：构造两路分支的 target/fallthrough 落点对。
  /// `target_taken_on_true` 指出条件成立时走哪条边；条件已解析时另一边标记 dead。
  pub fn conditional_targets(
    &self,
    inst_op: BcOp,
    target: BcOp,
    cond: ConditionState,
    target_taken_on_true: bool,
  ) -> SmallVector<JumpTarget, 2> {
    LUAU_ASSERT!(target.kind == BcOpKind::Block);
    let inst_block = self.func.inst(inst_op).operator_deref().block;
    let fallthrough = self.get_fallthrough(inst_block);
    LUAU_ASSERT!(fallthrough.is_some());

    let (target_dead, fallthrough_dead) = match cond {
      ConditionState::AlwaysTrue => (!target_taken_on_true, target_taken_on_true),
      ConditionState::AlwaysFalse => (target_taken_on_true, !target_taken_on_true),
      ConditionState::Unknown => (false, false),
    };

    [
      JumpTarget {
        dead: target_dead,
        block_op: target,
        condition: cond,
      },
      JumpTarget {
        dead: fallthrough_dead,
        block_op: fallthrough.unwrap(),
        condition: cond,
      },
    ]
    .into_iter()
    .collect()
  }

  /// cpp `Sccp::jumpTargets`：按指令的跳转语义给出全部落点；空结果表示非分支指令。
  pub fn jump_targets(&mut self, inst_op: BcOp) -> SmallVector<JumpTarget, 2> {
    let (opcode, ops): (LuauOpcode, Vec<BcOp>) = {
      let inst = self.func.inst(inst_op);
      (
        inst.operator_deref().op,
        inst.operator_deref().ops.as_slice().to_vec(),
      )
    };
    let ops = ops.as_slice();

    match opcode {
      LuauOpcode::LOP_JUMP | LuauOpcode::LOP_JUMPBACK => {
        LUAU_ASSERT!(ops[0].kind == BcOpKind::Block);
        [JumpTarget {
          dead: false,
          block_op: ops[0],
          condition: ConditionState::AlwaysTrue,
        }]
        .into_iter()
        .collect()
      }
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let cond = self.evaluate_condition(ops[0]);
        self.conditional_targets(inst_op, ops[1], cond, opcode == LuauOpcode::LOP_JUMPIF)
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        let cond = self.evaluate_comparison_condition(opcode, ops[0], ops[1]);
        let negated = matches!(
          opcode,
          LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPIFNOTLE | LuauOpcode::LOP_JUMPIFNOTLT
        );
        self.conditional_targets(inst_op, ops[2], cond, !negated)
      }
      LuauOpcode::LOP_JUMPXEQKNIL
      | LuauOpcode::LOP_JUMPXEQKB
      | LuauOpcode::LOP_JUMPXEQKN
      | LuauOpcode::LOP_JUMPXEQKS => {
        let cond = self.evaluate_xeqk_condition(inst_op);
        let neg_imm = *self.func.imm(ops[1]).operator_deref();
        LUAU_ASSERT!(neg_imm.kind == BcImmKind::Boolean);
        // Safety: 断言已限定活跃字段
        let negated = unsafe { neg_imm.value.value_boolean };
        self.conditional_targets(inst_op, ops[2], cond, !negated)
      }
      LuauOpcode::LOP_FORNPREP
      | LuauOpcode::LOP_FORNLOOP
      | LuauOpcode::LOP_FORGPREP
      | LuauOpcode::LOP_FORGPREP_NEXT
      | LuauOpcode::LOP_FORGPREP_INEXT => {
        self.conditional_targets(inst_op, ops[3], ConditionState::Unknown, true)
      }
      // FORGLOOP 比 FORGPREP* 多两个 imm 前导操作数，target 在 ops[5]
      LuauOpcode::LOP_FORGLOOP => {
        self.conditional_targets(inst_op, ops[5], ConditionState::Unknown, true)
      }
      LuauOpcode::LOP_CMPPROTO => {
        self.conditional_targets(inst_op, ops[2], ConditionState::Unknown, true)
      }
      LuauOpcode::LOP_JUMPX => {
        LUAU_ASSERT!(false, "Should have never parsed this");
        SmallVector::new()
      }
      _ => SmallVector::new(),
    }
  }
}
