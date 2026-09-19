use ulua_common::{
  enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT,
  records::small_vector::SmallVector,
};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{ConditionState, Constness, Sccp, VmConstOps},
  },
};

/// cpp 注释：LOADN 载荷为 16 位有符号，replaceUses 阶段以 LOADN 重写
const K_LOADN_MIN: i64 = i16::MIN as i64;
const K_LOADN_MAX: i64 = i16::MAX as i64;

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `SccpInterpreter::evaluateArith`：双 Imm 走整数折叠，双 VmConst 交由实现折叠。
  pub fn evaluate_arith(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    let ops = self.func.inst(inst_op).operator_deref().ops.clone();
    let ops = ops.as_slice();
    let (lhs, rhs) = (ops[0], ops[1]);

    let lhs_constness = self.state.operand_lattice(lhs);
    let rhs_constness = self.state.operand_lattice(rhs);

    match (lhs_constness, rhs_constness) {
      (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) => {
        if lhs_imm.kind != BcImmKind::Int || rhs_imm.kind != BcImmKind::Int {
          return Constness::NotAConstant;
        }
        // Safety: kind == Int 时 value_int 为活跃字段
        let (lv, rv) = unsafe { (lhs_imm.value.value_int, rhs_imm.value.value_int) };

        // 除/模/幂不能折叠：除零未定义，DIV/POW 为浮点而 BcImm 只能表示整数
        if rv == 0
          && matches!(
            op,
            LuauOpcode::LOP_DIV | LuauOpcode::LOP_MOD | LuauOpcode::LOP_IDIV
          )
        {
          return Constness::NotAConstant;
        }
        if matches!(op, LuauOpcode::LOP_DIV | LuauOpcode::LOP_POW) {
          return Constness::NotAConstant;
        }

        let lv = i64::from(lv);
        let rv = i64::from(rv);
        let result: i64 = match op {
          LuauOpcode::LOP_ADD => lv + rv,
          LuauOpcode::LOP_SUB => lv - rv,
          LuauOpcode::LOP_MUL => lv * rv,
          LuauOpcode::LOP_MOD => {
            // Lua 取模：结果符号随除数
            let mut remainder = lv % rv;
            if remainder != 0 && (lv < 0) != (rv < 0) {
              remainder += rv;
            }
            remainder
          }
          LuauOpcode::LOP_IDIV => {
            // Lua 整除：向负无穷取整
            let mut quotient = lv / rv;
            if quotient < 0 && lv % rv != 0 {
              quotient -= 1;
            }
            quotient
          }
          _ => {
            LUAU_ASSERT!(false, "Unhandled opcode");
            return Constness::NotAConstant;
          }
        };

        if !(K_LOADN_MIN..=K_LOADN_MAX).contains(&result) {
          return Constness::NotAConstant;
        }

        Constness::ImmConstant(self.vm_ops.make_imm_int(result as i32))
      }
      (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) => {
        match self.vm_ops.evaluate(self.func, lhs_vm, rhs_vm, op) {
          Some(vm_const) => Constness::VmConstant(vm_const),
          None => Constness::NotAConstant,
        }
      }
      (Constness::Undetermined, Constness::Undetermined) => Constness::Undetermined,
      _ => Constness::NotAConstant,
    }
  }

  /// cpp `SccpInterpreter::evaluate`：单条指令的常量求值。
  pub fn evaluate(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    let ops: SmallVector<_, 4> = self.func.inst(inst_op).operator_deref().ops.clone();
    let ops: &[BcOp] = ops.as_slice();

    match op {
      LuauOpcode::LOP_LOADK | LuauOpcode::LOP_LOADKX => {
        LUAU_ASSERT!(ops[0].kind == BcOpKind::VmConst);
        Constness::VmConstant(ops[0])
      }
      LuauOpcode::LOP_LOADB | LuauOpcode::LOP_LOADN => {
        LUAU_ASSERT!(ops[0].kind == BcOpKind::Imm);
        Constness::ImmConstant(self.vm_ops.as_imm(self.func, ops[0]))
      }
      LuauOpcode::LOP_LOADNIL => {
        let nil_const = self.vm_ops.make_nil(self.func);
        Constness::VmConstant(nil_const)
      }
      LuauOpcode::LOP_ADD
      | LuauOpcode::LOP_SUB
      | LuauOpcode::LOP_MUL
      | LuauOpcode::LOP_DIV
      | LuauOpcode::LOP_MOD
      | LuauOpcode::LOP_POW
      | LuauOpcode::LOP_IDIV => self.evaluate_arith(op, inst_op),
      LuauOpcode::LOP_MOVE => self.state.operand_lattice(ops[0]),
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let cond = self.evaluate_condition(ops[0]);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[ops[0]]);
        }
        let jumps_on_true = op == LuauOpcode::LOP_JUMPIF;
        let takes_jump = (cond == ConditionState::AlwaysTrue) == jumps_on_true;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        let cond = self.evaluate_comparison_condition(op, ops[0], ops[1]);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[ops[0], ops[1]]);
        }
        let negated = matches!(
          op,
          LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPIFNOTLE | LuauOpcode::LOP_JUMPIFNOTLT
        );
        let takes_jump = (cond == ConditionState::AlwaysTrue) != negated;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      LuauOpcode::LOP_JUMPXEQKNIL
      | LuauOpcode::LOP_JUMPXEQKB
      | LuauOpcode::LOP_JUMPXEQKN
      | LuauOpcode::LOP_JUMPXEQKS => {
        let cond = self.evaluate_xeqk_condition(inst_op);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[ops[0]]);
        }
        // 取反位以 Imm(bool) 编码，falsey 即其真值
        let negated = !self.vm_ops.falsey(self.func, ops[1]);
        let takes_jump = (cond == ConditionState::AlwaysTrue) != negated;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      _ => Constness::NotAConstant,
    }
  }
}
