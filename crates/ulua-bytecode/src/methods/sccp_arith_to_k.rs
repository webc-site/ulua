use alloc::vec::Vec;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    sccp::{Constness, ConstnessLattice, Sccp, VmConstOps},
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
    let old_ops: Vec<BcOp> = self.func.inst(op).operator_deref().ops.as_slice().to_vec();
    for old in old_ops {
      self.state.erase_use(op, old);
    }
    {
      let inst = self.func.inst_op(op);
      inst.ops.clear();
      for &new_op in new_ops {
        inst.ops.push_back(new_op);
      }
    }
    for &new_op in new_ops {
      self.state.record_use(new_op, op);
    }
  }

  /// 一侧为算术常量（number 类 VM 常量）
  fn is_const_number(&mut self, lattice: &ConstnessLattice) -> bool {
    lattice.kind == Constness::VmConstant
      && lattice.vm_const.is_some()
      && self
        .vm_ops
        .is_arithmetic_constant(self.func, lattice.vm_const.unwrap())
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
    let block_count = self.func.blocks.len();
    for block_idx in 0..block_count {
      let block_idx = block_idx as u32;
      if self.state.block_uses.get_or_insert(block_idx).is_empty() {
        continue;
      }

      let block_ops: Vec<BcOp> = self.func.blocks[block_idx as usize]
        .ops
        .iter()
        .cloned()
        .collect();
      let mut to_erase: Vec<BcOp> = Vec::new();

      for op in block_ops {
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
        let (non_constant_op, constant_k, rk) =
          if self.is_const_number(&rhs_lat) && lhs_lat.kind == Constness::NotAConstant {
            (lhs, rhs_lat, false)
          } else if self.is_const_number(&lhs_lat) && rhs_lat.kind == Constness::NotAConstant {
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
            (rhs, lhs_lat, rk)
          } else {
            continue;
          };

        let prev_const_operand = if non_constant_op == lhs { rhs } else { lhs };
        let constant_is_rhs = non_constant_op == lhs;

        // 一侧恒常时代数折叠：加零、乘零/一、零/一次幂等
        let value_number = self
          .vm_ops
          .as_number(self.func, constant_k.vm_const.unwrap());
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
            self.set_ops(op, &[non_constant_op, constant_k.vm_const.unwrap()]);
          } else {
            // SUBRK/DIVRK 期望 B 为常量表索引
            self.set_ops(op, &[constant_k.vm_const.unwrap(), non_constant_op]);
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
