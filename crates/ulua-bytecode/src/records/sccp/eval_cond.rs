use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use super::{ConditionState, Constness, Sccp, VmConstOps, three_way};
use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::bc_op::BcOp,
};

// ── abs-r139：并自 `methods/sccp_evaluate_condition.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// 布尔结果映射到格态（cpp `condTrue ? AlwaysTrue : AlwaysFalse`）。
  fn resolved(cond: bool) -> ConditionState {
    if cond {
      ConditionState::AlwaysTrue
    } else {
      ConditionState::AlwaysFalse
    }
  }

  /// cpp `SccpInterpreter::evaluateCondition`：JUMPIF* 条件的真值。
  /// cpp 对一切 Constant（vmConst/immConst）统一走 `falsey`：仅 nil 与
  /// false 为假，imm 侧无 nil，故 Boolean imm 取自身真值，其余恒真
  /// （`LOADN r,0; JUMPIF r` 的整数 imm 折 AlwaysTrue，与 cpp 一致）。
  pub(crate) fn evaluate_condition(&mut self, op: BcOp) -> ConditionState {
    let lattice = self.state.operand_lattice(op);
    match lattice {
      Constness::VmConstant(vm_const) => Self::resolved(!self.vm_ops.falsey(self.func, vm_const)),
      Constness::ImmConstant(imm) => Self::resolved(match imm.kind() {
        BcImmKind::Boolean => imm.as_boolean(),
        _ => true,
      }),
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
  pub(crate) fn evaluate_comparison_condition(
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
      Constness::ImmConstant(imm) => imm.kind() == BcImmKind::Int,
      _ => false,
    };

    if is_ordering_op && (!orderable(lhs_const, self) || !orderable(rhs_const, self)) {
      return ConditionState::Unknown;
    }

    // VM 常量种类不匹配：序无定义（cpp compare 不可达），但等值有定义——
    // cpp `eq` 对异种类 VmConst 对直接返回 false（Sccp.cpp:157-158），故
    // 等值算符折 AlwaysFalse、序算符落 Unknown
    if let (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) = (lhs_const, rhs_const)
      && !self.vm_ops.kind_equals(self.func, lhs_vm, rhs_vm)
    {
      return if is_ordering_op {
        ConditionState::Unknown
      } else {
        ConditionState::AlwaysFalse
      };
    }

    match (lhs_const, rhs_const) {
      (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) => {
        // 同种类但无定义关系的对（Vector/Table/Closure…）返回 None 落
        // Unknown，不得当"相等"折叠（cpp eq 返回 nullopt）
        let Some(cmp) = self.vm_ops.cmp_ops(self.func, lhs_vm, rhs_vm) else {
          return ConditionState::Unknown;
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) => {
        if lhs_imm.kind() == BcImmKind::Int && rhs_imm.kind() == BcImmKind::Int {
          let (lv, rv) = (lhs_imm.as_int(), rhs_imm.as_int());
          let cmp = three_way(lv, rv);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else if lhs_imm.kind() == BcImmKind::Boolean && rhs_imm.kind() == BcImmKind::Boolean {
          let (lb, rb) = (lhs_imm.as_boolean(), rhs_imm.as_boolean());
          let cmp = i32::from(lb != rb);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else {
          ConditionState::Unknown
        }
      }
      // 混合 VmConst×Imm 臂：cpp `eq` 对不可比较种类对（String×Int-imm 等）
      // 返回 nullopt 落 Unknown；cmp_imm 返回 None 时同样不得默认"相等"
      (Constness::VmConstant(vm_const), Constness::ImmConstant(imm)) => {
        let Some(cmp) = self.vm_ops.cmp_imm(self.func, vm_const, imm) else {
          return ConditionState::Unknown;
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(imm), Constness::VmConstant(vm_const)) => {
        let Some(cmp) = self.vm_ops.cmp_imm(self.func, vm_const, imm) else {
          return ConditionState::Unknown;
        };
        // NaN 参与时 cmp_imm 返回哨兵 1（eq/lt/le 基础真值全 false，cpp
        // bcCompare 逐算符独立判 false）；此处不得取反——`-1` 会使
        // `imm < NaN` / `imm <= NaN` 误折恒真（miscompile）
        let cmp = if self.vm_ops.is_nan_const(self.func, vm_const) {
          cmp
        } else {
          -cmp
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      _ => ConditionState::Unknown,
    }
  }

  /// cpp `SccpInterpreter::evaluateXeqkCondition`。
  pub(crate) fn evaluate_xeqk_condition(&mut self, inst_op: BcOp) -> ConditionState {
    let inst = self.func.inst(inst_op);
    let inst = inst.operator_deref();
    let opcode = inst.op;
    let ops: &[BcOp] = inst.ops.as_slice();

    let val_const = self.state.operand_lattice(ops[0]);

    match opcode {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        if let Constness::VmConstant(vm) = val_const {
          let nil = self.vm_ops.make_nil(self.func);
          // 纯读查询一次即可：kind_equals 无副作用，重复调用只多一遍解引用
          let same_kind_as_nil = self.vm_ops.kind_equals(self.func, vm, nil);
          // 既 falsey 又与 nil 同类：条件恒真
          if self.vm_ops.falsey(self.func, vm) && same_kind_as_nil {
            return ConditionState::AlwaysTrue;
          }
          // 与 nil 不同类：条件恒假
          if !same_kind_as_nil {
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
          Constness::ImmConstant(imm) if imm.kind() == BcImmKind::Boolean => {
            let lhs_bool = imm.as_boolean();
            let rhs_imm = self.vm_ops.as_imm(self.func, cmp_imm_op);
            // JUMPXEQKB 的比较操作数恒为 Boolean imm（图布局约定）
            let rhs_bool = rhs_imm.as_boolean();
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
            if opcode == LuauOpcode::LOP_JUMPXEQKN && imm.kind() == BcImmKind::Int =>
          {
            let value = imm.as_int();
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
