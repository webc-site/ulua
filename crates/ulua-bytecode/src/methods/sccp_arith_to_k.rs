use std::vec::Vec;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    sccp::{Constness, Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `Sccp::arithToKOpcode`：算术 opcode 对应的 K 变体。
  fn arith_to_k_opcode(op: LuauOpcode) -> Option<LuauOpcode> {
    match op {
      LuauOpcode::LOP_ADD => Some(LuauOpcode::LOP_ADDK),
      LuauOpcode::LOP_SUB => Some(LuauOpcode::LOP_SUBK),
      LuauOpcode::LOP_MUL => Some(LuauOpcode::LOP_MULK),
      LuauOpcode::LOP_DIV => Some(LuauOpcode::LOP_DIVK),
      LuauOpcode::LOP_MOD => Some(LuauOpcode::LOP_MODK),
      LuauOpcode::LOP_POW => Some(LuauOpcode::LOP_POWK),
      _ => None,
    }
  }

  /// cpp `Sccp::isPureProducer`：无使用即可删除的纯值生产者。
  fn is_pure_producer(op: LuauOpcode) -> bool {
    matches!(
      op,
      LuauOpcode::LOP_LOADK
        | LuauOpcode::LOP_LOADKX
        | LuauOpcode::LOP_LOADN
        | LuauOpcode::LOP_LOADB
        | LuauOpcode::LOP_LOADNIL
        | LuauOpcode::LOP_GETUPVAL
    )
  }

  /// 整型立即数快捷构造（对应 cpp `BcImm{BcImmKind::Int}` 字面量）
  fn int_imm(value: i32) -> BcImm {
    BcImm {
      kind: BcImmKind::Int,
      value: BcImmValue { value_int: value },
    }
  }

  /// cpp `Sccp::setOps`：替换指令操作数，同步 def→use 反向边。
  fn set_ops(&mut self, op: BcOp, new_ops: &[BcOp]) {
    // 拆分借用：旧 ops 的遍历只读 func，erase_use/record_use 只写 state，
    // 两字段不相交，无需旧实现的 to_vec 快照
    let Sccp { func, state, .. } = self;
    {
      let inst = func.inst(op);
      for old in inst.operator_deref().ops.iter().copied() {
        state.erase_use(op, old);
      }
    }
    let inst = func.inst_op(op);
    inst.ops.clear();
    inst.ops.extend(new_ops.iter().copied());
    for &new_op in new_ops {
      state.record_use(new_op, op);
    }
  }

  /// 一侧为算术常量（number 类 VM 常量）时返回该常量 op（cpp `isConstNumber`）。
  fn const_number(&mut self, lattice: &Constness) -> Option<BcOp> {
    let Constness::VmConstant(const_op) = *lattice else {
      return None;
    };
    self
      .vm_ops
      .is_arithmetic_constant(self.func, const_op)
      .then_some(const_op)
  }

  /// cpp `Sccp::eraseDeadProducer`：删除无引用的纯值生产者。
  fn erase_dead_producer(&mut self, op: BcOp) {
    if op.kind != BcOpKind::Inst {
      return;
    }
    let (opcode, block) = {
      let inst = self.func.inst(op);
      (inst.operator_deref().op, inst.operator_deref().block)
    };
    if !Self::is_pure_producer(opcode) {
      return;
    }
    if !self.state.uses_of(op).is_empty() {
      return;
    }
    self.func.block_op(block).ops.retain(|x| *x != op);
  }

  /// cpp `Sccp::arithToK`：算术一侧为已知常量时改写为 K/RK 变体，并顺带做代数折叠。
  pub(crate) fn arith_to_k(&mut self) {
    for block_idx in 0..self.func.blocks.len() {
      if self
        .state
        .block_uses
        .get_or_insert(block_idx as u32)
        .is_empty()
      {
        continue;
      }

      // 本块的改写会 retain/clear 块的 ops，需先快照；借用 Sccp::scratch
      // 复用缓冲（同 sccp_visit 约定），避免逐块分配。用下标遍历，
      // 以免 scratch 的共享借用与 self 的可变借用冲突。
      self.scratch.ops.clear();
      self
        .scratch
        .ops
        .extend(self.func.blocks[block_idx].ops.iter().copied());

      let mut to_erase: Vec<BcOp> = Vec::new();

      for k in 0..self.scratch.ops.len() {
        let op = self.scratch.ops[k];
        let opcode = self.func.inst(op).operator_deref().op;
        let Some(mut k_opcode) = Self::arith_to_k_opcode(opcode) else {
          continue;
        };
        // cpp（Sccp.h arithToK）：先验证双操作数形状，再解引用操作数
        let (lhs, rhs) = {
          let inst = self.func.inst(op);
          let ops = inst.operator_deref().ops.as_slice();
          if ops.len() != 2 {
            continue;
          }
          (ops[0], ops[1])
        };

        let lhs_lat = self.state.operand_lattice(lhs);
        let rhs_lat = self.state.operand_lattice(rhs);

        // 至多一侧为 VmConstant：双侧恒常时算术已被整体折叠
        let (non_constant_op, constant_op, rk) = if let Some(k) = self.const_number(&rhs_lat)
          && matches!(lhs_lat, Constness::NotAConstant)
        {
          (lhs, k, false)
        } else if let Some(k) = self.const_number(&lhs_lat)
          && matches!(rhs_lat, Constness::NotAConstant)
        {
          if !matches!(
            opcode,
            LuauOpcode::LOP_ADD | LuauOpcode::LOP_MUL | LuauOpcode::LOP_SUB | LuauOpcode::LOP_DIV
          ) {
            continue;
          }
          if opcode == LuauOpcode::LOP_SUB {
            k_opcode = LuauOpcode::LOP_SUBRK;
          } else if opcode == LuauOpcode::LOP_DIV {
            k_opcode = LuauOpcode::LOP_DIVRK;
          }
          let rk = matches!(k_opcode, LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK);
          (rhs, k, rk)
        } else {
          continue;
        };

        let prev_const_operand = if non_constant_op == lhs { rhs } else { lhs };
        let constant_is_rhs = non_constant_op == lhs;

        // 一侧恒常时代数折叠：加零、乘零/一、零/一次幂等
        let value_number = self.vm_ops.as_number(self.func, constant_op);
        if value_number == 0.0 {
          if opcode == LuauOpcode::LOP_ADD || (opcode == LuauOpcode::LOP_SUB && constant_is_rhs) {
            self.func.inst_op(op).op = LuauOpcode::LOP_MOVE;
            self.set_ops(op, &[non_constant_op]);
          } else if opcode == LuauOpcode::LOP_MUL {
            self.func.inst_op(op).op = LuauOpcode::LOP_LOADN;
            let imm_op = self.func.add_imm_alt_b(Self::int_imm(0));
            self.set_ops(op, &[imm_op]);
          } else if opcode == LuauOpcode::LOP_POW && constant_is_rhs {
            // x ^ 0 == 1（0 ^ x 不折叠：x != 0 时为 0）
            self.func.inst_op(op).op = LuauOpcode::LOP_LOADN;
            let imm_op = self.func.add_imm_alt_b(Self::int_imm(1));
            self.set_ops(op, &[imm_op]);
          }
        } else if value_number == 1.0 {
          if opcode == LuauOpcode::LOP_MUL
            || (opcode == LuauOpcode::LOP_POW && constant_is_rhs)
            || (opcode == LuauOpcode::LOP_DIV && constant_is_rhs)
          {
            self.func.inst_op(op).op = LuauOpcode::LOP_MOVE;
            self.set_ops(op, &[non_constant_op]);
          }
        } else {
          self.func.inst_op(op).op = k_opcode;
          if !rk {
            self.set_ops(op, &[non_constant_op, constant_op]);
          } else {
            // SUBRK/DIVRK 期望 B 为常量表索引
            self.set_ops(op, &[constant_op, non_constant_op]);
          }
        }

        to_erase.push(prev_const_operand);
      }

      for op in to_erase {
        self.erase_dead_producer(op);
      }
    }
  }
}
