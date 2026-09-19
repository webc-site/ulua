use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{ConditionState, Constness, Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// 布尔结果映射到格态（cpp `condTrue ? AlwaysTrue : AlwaysFalse`）。
  fn resolved(cond: bool) -> ConditionState {
    if cond {
      ConditionState::AlwaysTrue
    } else {
      ConditionState::AlwaysFalse
    }
  }

  /// cpp `SccpInterpreter::evaluateCondition`：JUMPIF* 条件的真值。
  pub fn evaluate_condition(&mut self, op: BcOp) -> ConditionState {
    let lattice = self.state.operand_lattice(op);
    match lattice {
      Constness::VmConstant(vm_const) => Self::resolved(!self.vm_ops.falsey(self.func, vm_const)),
      Constness::ImmConstant(imm) if imm.kind == BcImmKind::Boolean => {
        // Safety: kind == Boolean 时 value_boolean 为活跃字段
        Self::resolved(unsafe { imm.value.value_boolean })
      }
      _ => ConditionState::Unknown,
    }
  }

  /// cpp `applyOp`：三路比较结果套用比较算符。
  fn apply_cmp(cmp: i32, op: LuauOpcode) -> bool {
    match op {
      LuauOpcode::LOP_JUMPIFEQ | LuauOpcode::LOP_JUMPIFNOTEQ => cmp == 0,
      LuauOpcode::LOP_JUMPIFLT | LuauOpcode::LOP_JUMPIFNOTLT => cmp < 0,
      LuauOpcode::LOP_JUMPIFLE | LuauOpcode::LOP_JUMPIFNOTLE => cmp <= 0,
      _ => {
        LUAU_ASSERT!(false, "Unhandled comparison opcode");
        false
      }
    }
  }

  /// cpp `SccpInterpreter::evaluateComparisonCondition`。
  pub fn evaluate_comparison_condition(
    &mut self,
    op: LuauOpcode,
    lhs: BcOp,
    rhs: BcOp,
  ) -> ConditionState {
    let lhs_const = self.state.operand_lattice(lhs);
    let rhs_const = self.state.operand_lattice(rhs);

    let is_ordering_op = matches!(
      op,
      LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFNOTLT
        | LuauOpcode::LOP_JUMPIFNOTLE
    );

    let orderable = |lat: Constness, sccp: &mut Self| match lat {
      Constness::VmConstant(vm_const) => sccp.vm_ops.is_orderable(sccp.func, vm_const),
      Constness::ImmConstant(imm) => imm.kind == BcImmKind::Int,
      _ => false,
    };

    if is_ordering_op && (!orderable(lhs_const, self) || !orderable(rhs_const, self)) {
      return ConditionState::Unknown;
    }

    // VM 常量种类不匹配时无定义的序/等关系
    if let (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) = (lhs_const, rhs_const)
      && !self.vm_ops.kind_equals(self.func, lhs_vm, rhs_vm)
    {
      return ConditionState::Unknown;
    }

    match (lhs_const, rhs_const) {
      (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) => {
        let cmp = self.vm_ops.cmp_ops(self.func, lhs_vm, rhs_vm);
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) => {
        if lhs_imm.kind == BcImmKind::Int && rhs_imm.kind == BcImmKind::Int {
          // Safety: kind == Int 时 value_int 为活跃字段
          let (lv, rv) = unsafe { (lhs_imm.value.value_int, rhs_imm.value.value_int) };
          let cmp = i32::from(lv > rv) - i32::from(lv < rv);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else if lhs_imm.kind == BcImmKind::Boolean && rhs_imm.kind == BcImmKind::Boolean {
          // Safety: kind == Boolean 时 value_boolean 为活跃字段
          let (lb, rb) = unsafe { (lhs_imm.value.value_boolean, rhs_imm.value.value_boolean) };
          let cmp = i32::from(lb != rb);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else {
          ConditionState::Unknown
        }
      }
      (Constness::VmConstant(vm_const), Constness::ImmConstant(imm)) => {
        let cmp = self.vm_ops.cmp_imm(self.func, vm_const, imm);
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(imm), Constness::VmConstant(vm_const)) => {
        let cmp = -self.vm_ops.cmp_imm(self.func, vm_const, imm);
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      _ => ConditionState::Unknown,
    }
  }

  /// cpp `SccpInterpreter::evaluateXeqkCondition`。
  pub fn evaluate_xeqk_condition(&mut self, inst_op: BcOp) -> ConditionState {
    let inst = self.func.inst(inst_op);
    let opcode = inst.operator_deref().op;
    let ops: &[BcOp] = inst.operator_deref().ops.as_slice();

    let val_const = self.state.operand_lattice(ops[0]);

    match opcode {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        if let Constness::VmConstant(vm) = val_const {
          let nil = self.vm_ops.make_nil(self.func);
          // 既 falsey 又与 nil 同类：条件恒真
          if self.vm_ops.falsey(self.func, vm) && self.vm_ops.kind_equals(self.func, vm, nil) {
            return ConditionState::AlwaysTrue;
          }
          // 与 nil 不同类：条件恒假
          if !self.vm_ops.kind_equals(self.func, vm, nil) {
            return ConditionState::AlwaysFalse;
          }
        } else if matches!(val_const, Constness::ImmConstant(_)) {
          return ConditionState::AlwaysFalse;
        }
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        let cmp_imm_op = ops[3];
        LUAU_ASSERT!(cmp_imm_op.kind == BcOpKind::Imm);
        match val_const {
          Constness::ImmConstant(imm) if imm.kind == BcImmKind::Boolean => {
            // Safety: kind == Boolean 时 value_boolean 为活跃字段
            let lhs_bool = unsafe { imm.value.value_boolean };
            let rhs_imm = self.vm_ops.as_imm(self.func, cmp_imm_op);
            // Safety: JUMPXEQKB 的取反位恒为 Boolean imm
            let rhs_bool = unsafe { rhs_imm.value.value_boolean };
            return Self::resolved(lhs_bool == rhs_bool);
          }
          Constness::VmConstant(vm) => {
            if let Some(eq) = self.vm_ops.eq_ops(self.func, vm, cmp_imm_op) {
              return Self::resolved(eq);
            }
          }
          _ => {}
        }
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        let cmp_const_op = ops[3];
        LUAU_ASSERT!(cmp_const_op.kind == BcOpKind::VmConst);
        match val_const {
          Constness::VmConstant(vm) => {
            if let Some(eq) = self.vm_ops.eq_ops(self.func, vm, cmp_const_op) {
              return Self::resolved(eq);
            }
          }
          // 整数 imm 只与 JUMPXEQKN 比较；字符串走 VmConstant 分支
          Constness::ImmConstant(imm)
            if opcode == LuauOpcode::LOP_JUMPXEQKN && imm.kind == BcImmKind::Int =>
          {
            // Safety: kind == Int 时 value_int 为活跃字段
            let value = unsafe { imm.value.value_int };
            if let Some(eq) = self.vm_ops.eq_int(self.func, cmp_const_op, value) {
              return Self::resolved(eq);
            }
          }
          _ => {}
        }
      }
      _ => {}
    }

    ConditionState::Unknown
  }
}
