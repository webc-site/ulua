use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use super::{ConditionState, Constness, Sccp, VmConstOps};
use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{bc_imm::BcImm, bc_op::BcOp},
};

// ── abs-r139：并自 `methods/sccp_evaluate_arith.rs` ──
/// cpp 注释：LOADN 载荷为 16 位有符号，replaceUses 阶段以 LOADN 重写
const K_LOADN_MIN: i64 = i16::MIN as i64;
const K_LOADN_MAX: i64 = i16::MAX as i64;

/// DIV/POW 的 double 结果仅当为有限整数时方可并入 LOADN 形态；inf/NaN/
/// 带小数部分一律返回 None（回落 Number VmConst 通路，oracle 同型）。
/// cast 饱和 + 射程过滤，超大整值（如 1e30）自然落 None。
fn int_if_representable(value: f64) -> Option<i64> {
  (value.is_finite() && value.fract() == 0.0).then_some(value as i64)
}

impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `SccpInterpreter::evaluateArith`：任意两个常量（Imm/VmConst 组合）
  /// 统一交由 `impl->evaluate` 升 double 折叠成 Number 型 VmConst
  /// （cpp/Bytecode/src/Sccp.cpp:240-262），并无整数折叠分支。
  ///
  /// DELIBERATE DEVIATION：Rust 版对双 Int-imm 且结果可整表示、落在 `LOADN`
  /// i16 载荷射程内的组合，另走整数折叠落 `LOADN`（而非 Number `LOADK`），
  /// 保住 Lua 的 integer 语义（`math.type` 不漂移）；该决策回归钉在
  /// tests/sccp_int_imm_fold.rs，防 sync-cpp 时被改回。其余一切常量对
  /// （超射程整对、DIV/POW 非整结果、除零 inf/NaN、Imm×VmConst 数值混合对）
  /// 与 cpp 同型走 `vm_ops.evaluate` 双精度通路——旧版对这些无条件
  /// NotAConstant，折叠强度低于 oracle，系偏差范围失控而非决策本身。
  pub(crate) fn evaluate_arith(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    // 双操作数均为 Copy：块作用域借切片拷出即释放 func 的共享借用，
    // 后续 &mut self.state 调用不再冲突，无需整条 ops clone
    let (lhs, rhs) = {
      let inst = self.func.inst(inst_op);
      let ops = inst.operator_deref().ops.as_slice();
      (ops[0], ops[1])
    };

    let lhs_constness = self.state.operand_lattice(lhs);
    let rhs_constness = self.state.operand_lattice(rhs);

    // 整数快路径只认双 Int-imm；落不进 LOADN 的组合不放弃，继续走下方
    // 双精度通路（oracle 对超射程整对照折 Number VmConst）
    if let (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) =
      (&lhs_constness, &rhs_constness)
      && lhs_imm.kind() == BcImmKind::Int
      && rhs_imm.kind() == BcImmKind::Int
      && let Some(imm) = self.fold_int_imm(op, lhs_imm.as_int(), rhs_imm.as_int())
    {
      return Constness::ImmConstant(imm);
    }

    match (lhs_constness.const_value(), rhs_constness.const_value()) {
      // cpp：双侧常量（任意 Imm/VmConst 组合）统一 evaluate，nullopt 落格底
      (Some(lhs_const), Some(rhs_const)) => {
        match self.vm_ops.evaluate(self.func, lhs_const, rhs_const, op) {
          Some(vm_const) => Constness::VmConstant(vm_const),
          None => Constness::NotAConstant,
        }
      }
      // cpp：双侧 Undetermined 才延续格顶
      (None, None)
        if matches!(lhs_constness, Constness::Undetermined)
          && matches!(rhs_constness, Constness::Undetermined) =>
      {
        Constness::Undetermined
      }
      _ => Constness::NotAConstant,
    }
  }

  /// 双 Int-imm 整数快路径（DELIBERATE DEVIATION 载体）：结果可整表示且落
  /// i16 射程 → Int imm；否则 None 由调用方回落双精度通路。ADD/SUB/MUL/
  /// MOD/IDIV 用 i64 精确整数域（操作数 |v|≤2^31，积 ≤2^62 不溢出）；
  /// DIV/POW 本征浮点，仅当 double 结果恰为有限整数时并入 LOADN 形态，
  /// 数值与 oracle 的 double 折叠一致，只是编码不同。
  fn fold_int_imm(&self, op: LuauOpcode, lv: i32, rv: i32) -> Option<BcImm> {
    let (l, r) = (i64::from(lv), i64::from(rv));
    // 负零守卫：整数域无法表示 -0.0，凡 double 真值为 -0.0 的组合必须落
    // 双精度通路（Number(-0.0)→LOADK），否则 `tostring` "-0" 漂移成 "0"，
    // 且后续 1/x 的 ±inf 方向翻转。-0.0 仅产自 MUL/DIV/IDIV 的 (0, 负)
    // 组合；ADD/SUB/MOD 的整值 0 恒为 +0.0（MOD 符号随除数，0%x = +0）
    let negative_zero = (l == 0 || r == 0) && (l < 0) != (r < 0);
    // MOD/IDIV 除零在整数域无定义，直接落双精度通路（NaN/inf，cpp 同款）
    let exact: Option<i64> = match op {
      LuauOpcode::LOP_ADD => Some(l + r),
      LuauOpcode::LOP_SUB => Some(l - r),
      LuauOpcode::LOP_MUL if negative_zero => None,
      LuauOpcode::LOP_MUL => Some(l * r),
      LuauOpcode::LOP_MOD => {
        // Lua 取模：结果符号随除数
        (r != 0).then(|| {
          let mut remainder = l % r;
          if remainder != 0 && (l < 0) != (r < 0) {
            remainder += r;
          }
          remainder
        })
      }
      LuauOpcode::LOP_IDIV => {
        // Lua 整除：向负无穷取整。`/` 向零截断，需 -1 修正的判据是
        // 余数非零且操作数符号相异（真实商为负）——不能用 `quotient < 0`：
        // 截断商为 0 时（如 -1 // 2 == -1）会漏修成 0
        if l == 0 && r < 0 {
          return None; // floor(-0.0) = -0.0，整数域不可表示
        }
        (r != 0).then(|| {
          let mut quotient = l / r;
          if l % r != 0 && (l < 0) != (r < 0) {
            quotient -= 1;
          }
          quotient
        })
      }
      LuauOpcode::LOP_DIV if l == 0 && r < 0 => None,
      LuauOpcode::LOP_DIV => int_if_representable(f64::from(lv) / f64::from(rv)),
      LuauOpcode::LOP_POW => int_if_representable(f64::from(lv).powf(f64::from(rv))),
      _ => {
        LUAU_ASSERT!(false, "Unhandled opcode");
        None
      }
    };

    exact
      .filter(|result| (K_LOADN_MIN..=K_LOADN_MAX).contains(result))
      .map(|result| self.vm_ops.make_imm_int(result as i32))
  }

  /// cpp `SccpInterpreter::evaluate`：单条指令的常量求值。
  pub(crate) fn evaluate(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    let (op0, op1) = {
      let inst = self.func.inst(inst_op);
      let s = inst.operator_deref().ops.as_slice();
      (
        s.first().copied().unwrap_or_default(),
        s.get(1).copied().unwrap_or_default(),
      )
    };

    match op {
      LuauOpcode::LOP_LOADK | LuauOpcode::LOP_LOADKX => {
        LUAU_ASSERT!(op0.kind == BcOpKind::VmConst);
        Constness::VmConstant(op0)
      }
      LuauOpcode::LOP_LOADB | LuauOpcode::LOP_LOADN => {
        LUAU_ASSERT!(op0.kind == BcOpKind::Imm);
        Constness::ImmConstant(self.vm_ops.as_imm(self.func, op0))
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
      LuauOpcode::LOP_MOVE => self.state.operand_lattice(op0),
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let cond = self.evaluate_condition(op0);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[op0]);
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
        let cond = self.evaluate_comparison_condition(op, op0, op1);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[op0, op1]);
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
          return self.state.unknown_condition_constness(&[op0]);
        }
        // 取反位以 Imm(bool) 编码，falsey 即其真值
        let negated = !self.vm_ops.falsey(self.func, op1);
        let takes_jump = (cond == ConditionState::AlwaysTrue) != negated;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      _ => Constness::NotAConstant,
    }
  }
}
