use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use super::{BcVmConstImpl, ConstValue, VmConstOps, three_way};
use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind},
  records::{bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp, bc_vm_const::BcVmConst},
};

// ── abs-r139：并自 `methods/sccp_bc_vm_const_impl.rs` ──
/// 按值取出 VM 常量（避开 const_op 的可变借用）
fn const_at<'f>(func: &BcFunction<'f>, op: BcOp) -> BcVmConst<'f> {
  LUAU_ASSERT!(op.kind == BcOpKind::VmConst);
  func.constants[op.index as usize]
}

/// 按值取出立即数
fn imm_at(func: &BcFunction<'_>, op: BcOp) -> BcImm {
  LUAU_ASSERT!(op.kind == BcOpKind::Imm);
  func.immediates[op.index as usize]
}

/// cpp `isNumber`+`asNumber` 合一：数值常量（Imm-Int / VmConst-Number）取
/// double 值，非数值返回 None（cpp 先 isNumber 守卫再断言取值，此处以
/// Option 收敛同一契约）
fn number_of(func: &BcFunction<'_>, value: ConstValue) -> Option<f64> {
  match value {
    ConstValue::Imm(imm) => (imm.kind() == BcImmKind::Int).then(|| f64::from(imm.as_int())),
    ConstValue::Vm(op) => {
      let c = const_at(func, op);
      (c.kind() == BcVmConstKind::Number).then(|| c.as_number())
    }
  }
}

impl BcVmConstImpl {
  /// cpp `findOrAddConst`：复用已存在的相等常量，否则追加。
  fn find_or_add_const<'f>(func: &mut BcFunction<'f>, value: BcVmConst<'f>) -> BcOp {
    match func
      .constants
      .iter()
      .position(|existing| *existing == value)
    {
      Some(idx) => BcOp::with(BcOpKind::VmConst, idx as u32),
      None => func.add_const(value),
    }
  }
}

impl VmConstOps for BcVmConstImpl {
  fn evaluate(
    &self,
    func: &mut BcFunction<'_>,
    lhs: ConstValue,
    rhs: ConstValue,
    op: LuauOpcode,
  ) -> Option<BcOp> {
    // cpp `isNumber`+`asNumber`（Sccp.cpp:189-227）：Imm-Int 与 VmConst-Number
    // 任意组合均为数值常量，升 double 折叠；其余种类（含 VmConst-Integer/
    // String/Boolean 与 Imm-Boolean）`isNumber` 为假 → nullopt，静默放弃折叠。
    let a = number_of(func, lhs)?;
    let b = number_of(func, rhs)?;
    let r = match op {
      LuauOpcode::LOP_ADD => a + b,
      LuauOpcode::LOP_SUB => a - b,
      LuauOpcode::LOP_MUL => a * b,
      LuauOpcode::LOP_DIV => a / b,
      // cpp 无 b==0 守卫（Sccp.h evaluateNumberBinaryOp）：除零求值落 NaN。
      // 注意 NaN 并不会真的折叠：BcVmConst::operator== 对 NaN 恒 false，
      // find_or_add_const 每次求值都追加新 NaN 条目，格在二次 visit 的
      // merge 处退化为 NotAConstant——MOD 与 cpp 同态地保留运行期判定，
      // 仅常量池残留死 NaN 条目（对照：DIV/IDIV 除零落 ±inf，inf==inf
      // 命中既有常量、格稳定，照常折叠）。单独拦除零反而是对 oracle 的
      // 无端偏差，见 tests/sccp_int_imm_fold.rs 的 MOD/DIV 除零回归。
      LuauOpcode::LOP_MOD => a - (a / b).floor() * b,
      LuauOpcode::LOP_POW => a.powf(b),
      LuauOpcode::LOP_IDIV => (a / b).floor(),
      _ => return None,
    };

    Some(Self::find_or_add_const(func, BcVmConst::Number(r)))
  }

  fn falsey(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    match op.kind {
      BcOpKind::VmConst => match const_at(func, op) {
        BcVmConst::Nil => true,
        BcVmConst::Boolean(value) => !value,
        _ => false,
      },
      BcOpKind::Imm => match imm_at(func, op) {
        BcImm::Boolean(value) => !value,
        _ => false,
      },
      _ => false,
    }
  }

  fn cmp_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<i32> {
    let lhs_const = const_at(func, lhs);
    let rhs_const = const_at(func, rhs);
    LUAU_ASSERT!(lhs_const.kind() == rhs_const.kind());
    match lhs_const.kind() {
      // NaN 与任何数（含 NaN）的 eq/lt/le 均为 false（cpp bcCompare 各算符
      // 独立判 false）。此哨兵是直接臂（Vm×Vm）的主语义，交换臂不经过此处；
      // three_way 的 PartialOrd 双假会误返 0（"相等"），故须先拦
      BcVmConstKind::Number => {
        let (a, b) = (lhs_const.as_number(), rhs_const.as_number());
        if a.is_nan() || b.is_nan() {
          Some(1)
        } else {
          Some(three_way(a, b))
        }
      }
      BcVmConstKind::Integer => Some(three_way(lhs_const.as_integer(), rhs_const.as_integer())),
      BcVmConstKind::Boolean => Some(i32::from(lhs_const.as_boolean() != rhs_const.as_boolean())),
      BcVmConstKind::String => Some(three_way(lhs_const.as_string(), rhs_const.as_string())),
      // Nil×Nil 相等（cpp eq 返回 true）；Vector/Vectord/Import/Table/
      // Closure/ClassShape 同种类对既无序也不可判等（cpp eq 返回 nullopt），
      // 返回 None 落 Unknown——旧实现 `_ => 0` 会把两个不同 Vector/不同
      // proto 的 Closure 折成"相等"，恒真/恒假跳转是 miscompile
      BcVmConstKind::Nil => Some(0),
      _ => None,
    }
  }

  fn is_nan_const(&self, func: &mut BcFunction<'_>, vm: BcOp) -> bool {
    let c = const_at(func, vm);
    c.kind() == BcVmConstKind::Number && c.as_number().is_nan()
  }

  fn cmp_imm(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcImm) -> Option<i32> {
    let lhs_const = const_at(func, lhs);
    match (lhs_const.kind(), rhs.kind()) {
      (BcVmConstKind::Number, BcImmKind::Int) => {
        // 直接臂（Vm×Int-imm）的主语义：NaN 返 1 令 eq/lt/le 全 false；
        // 交换臂（Imm×Vm）由调用方按 is_nan_const 跳过取反，勿改返回约定。
        // three_way 的 PartialOrd 双假会误返 0（"相等"）
        let a = lhs_const.as_number();
        if a.is_nan() {
          Some(1)
        } else {
          Some(three_way(a, f64::from(rhs.as_int())))
        }
      }
      (BcVmConstKind::Integer, BcImmKind::Int) => {
        Some(three_way(lhs_const.as_integer(), i64::from(rhs.as_int())))
      }
      (BcVmConstKind::Boolean, BcImmKind::Boolean) => {
        Some(i32::from(lhs_const.as_boolean() != rhs.as_boolean()))
      }
      // String/Nil/… × Int-imm 等异种类对无相等/序关系，必须返回 None 走
      // Unknown；旧实现 `_ => 0` 会把它们当"相等"折叠成恒真/恒假跳转
      // （cpp Sccp.cpp `eq` 对此类对返回 nullopt）
      _ => None,
    }
  }

  fn make_nil(&self, func: &mut BcFunction<'_>) -> BcOp {
    Self::find_or_add_const(func, BcVmConst::new())
  }

  fn make_imm_bool(&self, value: bool) -> BcImm {
    BcImm::Boolean(value)
  }

  fn make_imm_int(&self, value: i32) -> BcImm {
    BcImm::Int(value)
  }

  fn is_orderable(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    matches!(
      const_at(func, op).kind(),
      BcVmConstKind::Number | BcVmConstKind::Integer | BcVmConstKind::String
    )
  }

  fn kind_equals(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> bool {
    const_at(func, lhs).kind() == const_at(func, rhs).kind()
  }

  fn eq_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<bool> {
    match (lhs.kind, rhs.kind) {
      (BcOpKind::VmConst, BcOpKind::VmConst) => {
        let lhs_const = const_at(func, lhs);
        let rhs_const = const_at(func, rhs);
        match (lhs_const.kind(), rhs_const.kind()) {
          (BcVmConstKind::Number, BcVmConstKind::Number) => {
            Some(lhs_const.as_number() == rhs_const.as_number())
          }
          (BcVmConstKind::Integer, BcVmConstKind::Integer) => {
            Some(lhs_const.as_integer() == rhs_const.as_integer())
          }
          (BcVmConstKind::Number, BcVmConstKind::Integer) => {
            Some(lhs_const.as_number() == rhs_const.as_integer() as f64)
          }
          (BcVmConstKind::Integer, BcVmConstKind::Number) => {
            Some(lhs_const.as_integer() as f64 == rhs_const.as_number())
          }
          (BcVmConstKind::String, BcVmConstKind::String) => {
            Some(lhs_const.as_string() == rhs_const.as_string())
          }
          _ => None,
        }
      }
      (BcOpKind::VmConst, BcOpKind::Imm) => {
        let lhs_const = const_at(func, lhs);
        let rhs_imm = imm_at(func, rhs);
        if lhs_const.kind() == BcVmConstKind::Boolean && rhs_imm.kind() == BcImmKind::Boolean {
          Some(lhs_const.as_boolean() == rhs_imm.as_boolean())
        } else {
          None
        }
      }
      // (Imm, Imm) 与 (Imm, VmConst) 臂已删：仅有的两个调用点
      // （evaluate_xeqk_condition 的 JUMPXEQKB/KN/KS）lhs 恒为 VmConstant
      _ => None,
    }
  }

  fn eq_int(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: i32) -> Option<bool> {
    let lhs_const = const_at(func, lhs);
    match lhs_const.kind() {
      BcVmConstKind::Number => Some(f64::from(rhs) == lhs_const.as_number()),
      BcVmConstKind::Integer => Some(i64::from(rhs) == lhs_const.as_integer()),
      _ => None,
    }
  }

  fn is_arithmetic_constant(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    const_at(func, op).kind() == BcVmConstKind::Number
  }

  fn as_number(&self, func: &mut BcFunction<'_>, op: BcOp) -> f64 {
    let vm_const = const_at(func, op);
    LUAU_ASSERT!(vm_const.kind() == BcVmConstKind::Number);
    vm_const.as_number()
  }

  fn as_imm(&self, func: &mut BcFunction<'_>, op: BcOp) -> BcImm {
    imm_at(func, op)
  }
}
