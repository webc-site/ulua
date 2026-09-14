use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{ConditionState, Constness, ConstnessLattice, Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `SccpInterpreter::evaluateCondition`：JUMPIF* 条件的真值。
  pub fn evaluate_condition(&mut self, op: BcOp) -> ConditionState {
    let lattice = self.state.operand_lattice(op);
    match lattice.kind {
      Constness::VmConstant => {
        let vm_const = lattice.vm_const.unwrap();
        if self.vm_ops.falsey(self.func, vm_const) {
          ConditionState::AlwaysFalse
        } else {
          ConditionState::AlwaysTrue
        }
      }
      Constness::ImmConstant => {
        let imm = lattice.imm_const.unwrap();
        // Safety: kind 已限定活跃字段
        let value = unsafe { imm.value.value_boolean };
        if imm.kind == BcImmKind::Boolean {
          if !value {
            ConditionState::AlwaysFalse
          } else {
            ConditionState::AlwaysTrue
          }
        } else {
          ConditionState::Unknown
        }
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

    let orderable = |lat: &ConstnessLattice, sccp: &mut Self| match lat.kind {
      Constness::VmConstant => sccp.vm_ops.is_orderable(sccp.func, lat.vm_const.unwrap()),
      Constness::ImmConstant => lat.imm_const.unwrap().kind == BcImmKind::Int,
      _ => false,
    };

    if is_ordering_op && (!orderable(&lhs_const, self) || !orderable(&rhs_const, self)) {
      return ConditionState::Unknown;
    }

    // VM 常量种类不匹配时无定义的序/等关系
    if lhs_const.kind == Constness::VmConstant
      && rhs_const.kind == Constness::VmConstant
      && !self.vm_ops.kind_equals(
        self.func,
        lhs_const.vm_const.unwrap(),
        rhs_const.vm_const.unwrap(),
      )
    {
      return ConditionState::Unknown;
    }

    let resolved = |cond: bool| {
      if cond {
        ConditionState::AlwaysTrue
      } else {
        ConditionState::AlwaysFalse
      }
    };

    match (lhs_const.kind, rhs_const.kind) {
      (Constness::VmConstant, Constness::VmConstant) => {
        let cmp = self.vm_ops.cmp_ops(
          self.func,
          lhs_const.vm_const.unwrap(),
          rhs_const.vm_const.unwrap(),
        );
        resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant, Constness::ImmConstant) => {
        let lhs_imm = lhs_const.imm_const.unwrap();
        let rhs_imm = rhs_const.imm_const.unwrap();
        // Safety: kind 已限定活跃字段
        unsafe {
          if lhs_imm.kind == BcImmKind::Int && rhs_imm.kind == BcImmKind::Int {
            let (lv, rv) = (lhs_imm.value.value_int, rhs_imm.value.value_int);
            let cmp = i32::from(lv > rv) - i32::from(lv < rv);
            resolved(Self::apply_cmp(cmp, op))
          } else if lhs_imm.kind == BcImmKind::Boolean && rhs_imm.kind == BcImmKind::Boolean {
            let cmp = i32::from(lhs_imm.value.value_boolean != rhs_imm.value.value_boolean);
            resolved(Self::apply_cmp(cmp, op))
          } else {
            ConditionState::Unknown
          }
        }
      }
      (Constness::VmConstant, Constness::ImmConstant) => {
        let cmp = self.vm_ops.cmp_imm(
          self.func,
          lhs_const.vm_const.unwrap(),
          rhs_const.imm_const.unwrap(),
        );
        resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant, Constness::VmConstant) => {
        let cmp = -self.vm_ops.cmp_imm(
          self.func,
          rhs_const.vm_const.unwrap(),
          lhs_const.imm_const.unwrap(),
        );
        resolved(Self::apply_cmp(cmp, op))
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
        let nil_kind_equal = val_const.kind == Constness::VmConstant && {
          let vm = val_const.vm_const.unwrap();
          let nil = self.vm_ops.make_nil(self.func);
          self.vm_ops.falsey(self.func, vm) && self.vm_ops.kind_equals(self.func, vm, nil)
        };
        if nil_kind_equal {
          return ConditionState::AlwaysTrue;
        }
        let kind_mismatch = val_const.kind == Constness::ImmConstant
          || (val_const.kind == Constness::VmConstant && {
            let vm = val_const.vm_const.unwrap();
            let nil = self.vm_ops.make_nil(self.func);
            !self.vm_ops.kind_equals(self.func, vm, nil)
          });
        if kind_mismatch {
          return ConditionState::AlwaysFalse;
        }
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        let cmp_imm_op = ops[3];
        LUAU_ASSERT!(cmp_imm_op.kind == BcOpKind::Imm);
        if val_const.kind == Constness::ImmConstant {
          let imm = val_const.imm_const.unwrap();
          // Safety: kind 已限定活跃字段
          let lhs_bool = unsafe { imm.value.value_boolean };
          if imm.kind == BcImmKind::Boolean {
            let rhs_imm = self.vm_ops.as_imm(self.func, cmp_imm_op);
            // Safety: JUMPXEQKB 的取反位恒为 Boolean imm
            let rhs_bool = unsafe { rhs_imm.value.value_boolean };
            return if lhs_bool == rhs_bool {
              ConditionState::AlwaysTrue
            } else {
              ConditionState::AlwaysFalse
            };
          }
        } else if val_const.kind == Constness::VmConstant
          && let Some(eq) = self
            .vm_ops
            .eq_ops(self.func, val_const.vm_const.unwrap(), cmp_imm_op)
        {
          return if eq {
            ConditionState::AlwaysTrue
          } else {
            ConditionState::AlwaysFalse
          };
        }
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        let cmp_const_op = ops[3];
        LUAU_ASSERT!(cmp_const_op.kind == BcOpKind::VmConst);
        if val_const.kind == Constness::VmConstant {
          if let Some(eq) = self
            .vm_ops
            .eq_ops(self.func, val_const.vm_const.unwrap(), cmp_const_op)
          {
            return if eq {
              ConditionState::AlwaysTrue
            } else {
              ConditionState::AlwaysFalse
            };
          }
        } else if opcode == LuauOpcode::LOP_JUMPXEQKN && val_const.kind == Constness::ImmConstant {
          let imm = val_const.imm_const.unwrap();
          // Safety: kind 已限定活跃字段
          if imm.kind == BcImmKind::Int {
            let value = unsafe { imm.value.value_int };
            if let Some(eq) = self.vm_ops.eq_int(self.func, cmp_const_op, value) {
              return if eq {
                ConditionState::AlwaysTrue
              } else {
                ConditionState::AlwaysFalse
              };
            }
          }
        }
      }
      _ => {}
    }

    ConditionState::Unknown
  }
}
