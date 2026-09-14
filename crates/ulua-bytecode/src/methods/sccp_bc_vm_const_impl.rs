use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind},
  records::{
    bc_function::BcFunction,
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    bc_vm_const::{BcVmConst, BcVmConstValue},
    sccp::{BcVmConstImpl, VmConstOps},
  },
};

/// 按值取出 VM 常量（避开 const_op 的可变借用）
fn const_at(func: &BcFunction, op: BcOp) -> BcVmConst {
  LUAU_ASSERT!(op.kind == BcOpKind::VmConst);
  func.constants[op.index as usize]
}

/// 按值取出立即数
fn imm_at(func: &BcFunction, op: BcOp) -> BcImm {
  LUAU_ASSERT!(op.kind == BcOpKind::Imm);
  func.immediates[op.index as usize]
}

impl BcVmConstImpl {
  /// cpp `threeWay`：-1 / 0 / 1
  fn three_way<T: PartialOrd>(a: T, b: T) -> i32 {
    i32::from(a > b) - i32::from(a < b)
  }

  /// cpp `findOrAddConst`：复用已存在的相等常量，否则追加。
  fn find_or_add_const(func: &mut BcFunction, value: BcVmConst) -> BcOp {
    let found = func
      .constants
      .iter()
      .enumerate()
      .find(|(_, existing)| **existing == value)
      .map(|(idx, _)| idx);
    match found {
      Some(idx) => BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmConst, idx as u32),
      None => func.add_const(value),
    }
  }
}

impl VmConstOps for BcVmConstImpl {
  fn evaluate(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp, op: LuauOpcode) -> Option<BcOp> {
    let lhs_const = const_at(func, lhs);
    let rhs_const = const_at(func, rhs);

    // 算术折叠仅对两个同 kind 的 number 有定义；cpp（Sccp.cpp:36）显式
    // 防御两侧 kind 不一致的情形并静默放弃折叠，不 assert。
    if lhs_const.kind != rhs_const.kind || lhs_const.kind != BcVmConstKind::Number {
      return None;
    }

    // Safety: kind == Number 时 value_number 为活跃字段
    let (a, b) = unsafe { (lhs_const.value.value_number, rhs_const.value.value_number) };
    let r = match op {
      LuauOpcode::LOP_ADD => a + b,
      LuauOpcode::LOP_SUB => a - b,
      LuauOpcode::LOP_MUL => a * b,
      LuauOpcode::LOP_DIV => a / b,
      LuauOpcode::LOP_MOD => {
        if b == 0.0 {
          return None;
        }
        a - (a / b).floor() * b
      }
      LuauOpcode::LOP_POW => a.powf(b),
      LuauOpcode::LOP_IDIV => (a / b).floor(),
      _ => return None,
    };

    let result = BcVmConst {
      kind: BcVmConstKind::Number,
      value: BcVmConstValue { value_number: r },
    };
    Some(Self::find_or_add_const(func, result))
  }

  fn falsey(&self, func: &mut BcFunction, op: BcOp) -> bool {
    match op.kind {
      BcOpKind::VmConst => {
        let vm_const = const_at(func, op);
        // Safety: kind 已限定活跃字段
        unsafe {
          vm_const.kind == BcVmConstKind::Nil
            || (vm_const.kind == BcVmConstKind::Boolean && !vm_const.value.value_boolean)
        }
      }
      BcOpKind::Imm => {
        let imm = imm_at(func, op);
        // Safety: kind 已限定活跃字段
        unsafe { imm.kind == BcImmKind::Boolean && !imm.value.value_boolean }
      }
      _ => false,
    }
  }

  fn cmp_ops(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> i32 {
    let lhs_const = const_at(func, lhs);
    let rhs_const = const_at(func, rhs);
    LUAU_ASSERT!(lhs_const.kind == rhs_const.kind);
    // Safety: 两侧 kind 一致，读取对应活跃字段
    unsafe {
      match lhs_const.kind {
        BcVmConstKind::Number => {
          Self::three_way(lhs_const.value.value_number, rhs_const.value.value_number)
        }
        BcVmConstKind::Integer => {
          Self::three_way(lhs_const.value.value_integer, rhs_const.value.value_integer)
        }
        BcVmConstKind::Boolean => {
          i32::from(lhs_const.value.value_boolean != rhs_const.value.value_boolean)
        }
        BcVmConstKind::String => {
          Self::three_way(lhs_const.value.value_string, rhs_const.value.value_string)
        }
        _ => 0,
      }
    }
  }

  fn cmp_imm(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcImm) -> i32 {
    let lhs_const = const_at(func, lhs);
    // Safety: kind 已限定活跃字段
    unsafe {
      match (lhs_const.kind, rhs.kind) {
        (BcVmConstKind::Number, BcImmKind::Int) => {
          Self::three_way(lhs_const.value.value_number, f64::from(rhs.value.value_int))
        }
        (BcVmConstKind::Integer, BcImmKind::Int) => Self::three_way(
          lhs_const.value.value_integer,
          i64::from(rhs.value.value_int),
        ),
        (BcVmConstKind::Boolean, BcImmKind::Boolean) => {
          i32::from(lhs_const.value.value_boolean != rhs.value.value_boolean)
        }
        _ => 0,
      }
    }
  }

  fn make_nil(&self, func: &mut BcFunction) -> BcOp {
    Self::find_or_add_const(func, BcVmConst::new())
  }

  fn make_imm_bool(&self, value: bool) -> BcImm {
    BcImm {
      kind: BcImmKind::Boolean,
      value: BcImmValue {
        value_boolean: value,
      },
    }
  }

  fn make_imm_int(&self, value: i32) -> BcImm {
    BcImm {
      kind: BcImmKind::Int,
      value: BcImmValue { value_int: value },
    }
  }

  fn is_orderable(&self, func: &mut BcFunction, op: BcOp) -> bool {
    matches!(
      const_at(func, op).kind,
      BcVmConstKind::Number | BcVmConstKind::Integer | BcVmConstKind::String
    )
  }

  fn kind_equals(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> bool {
    const_at(func, lhs).kind == const_at(func, rhs).kind
  }

  fn eq_ops(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> Option<bool> {
    match (lhs.kind, rhs.kind) {
      (BcOpKind::VmConst, BcOpKind::VmConst) => {
        let lhs_const = const_at(func, lhs);
        let rhs_const = const_at(func, rhs);
        // Safety: kind 已限定活跃字段
        unsafe {
          match (lhs_const.kind, rhs_const.kind) {
            (BcVmConstKind::Number, BcVmConstKind::Number) => {
              Some(lhs_const.value.value_number == rhs_const.value.value_number)
            }
            (BcVmConstKind::Integer, BcVmConstKind::Integer) => {
              Some(lhs_const.value.value_integer == rhs_const.value.value_integer)
            }
            (BcVmConstKind::Number, BcVmConstKind::Integer) => {
              Some(lhs_const.value.value_number == rhs_const.value.value_integer as f64)
            }
            (BcVmConstKind::Integer, BcVmConstKind::Number) => {
              Some(lhs_const.value.value_integer as f64 == rhs_const.value.value_number)
            }
            (BcVmConstKind::String, BcVmConstKind::String) => {
              Some(lhs_const.value.value_string == rhs_const.value.value_string)
            }
            _ => None,
          }
        }
      }
      (BcOpKind::VmConst, BcOpKind::Imm) => {
        let lhs_const = const_at(func, lhs);
        let rhs_imm = imm_at(func, rhs);
        // Safety: kind 已限定活跃字段
        unsafe {
          if lhs_const.kind == BcVmConstKind::Boolean && rhs_imm.kind == BcImmKind::Boolean {
            Some(lhs_const.value.value_boolean == rhs_imm.value.value_boolean)
          } else {
            None
          }
        }
      }
      (BcOpKind::Imm, BcOpKind::Imm) => {
        let lhs_imm = imm_at(func, lhs);
        let rhs_imm = imm_at(func, rhs);
        // Safety: kind 已限定活跃字段
        unsafe {
          if lhs_imm.kind == BcImmKind::Boolean && rhs_imm.kind == BcImmKind::Boolean {
            Some(lhs_imm.value.value_boolean == rhs_imm.value.value_boolean)
          } else if lhs_imm.kind == BcImmKind::Int && rhs_imm.kind == BcImmKind::Int {
            Some(lhs_imm.value.value_int == rhs_imm.value.value_int)
          } else {
            None
          }
        }
      }
      _ => None,
    }
  }

  fn eq_bool(&self, func: &mut BcFunction, lhs: BcOp, rhs: bool) -> Option<bool> {
    let lhs_const = const_at(func, lhs);
    // Safety: kind 已限定活跃字段
    unsafe {
      if lhs_const.kind == BcVmConstKind::Boolean {
        Some(lhs_const.value.value_boolean == rhs)
      } else {
        None
      }
    }
  }

  fn eq_int(&self, func: &mut BcFunction, lhs: BcOp, rhs: i32) -> Option<bool> {
    let lhs_const = const_at(func, lhs);
    // Safety: kind 已限定活跃字段
    unsafe {
      match lhs_const.kind {
        BcVmConstKind::Number => Some(f64::from(rhs) == lhs_const.value.value_number),
        BcVmConstKind::Integer => Some(i64::from(rhs) == lhs_const.value.value_integer),
        _ => None,
      }
    }
  }

  fn is_arithmetic_constant(&self, func: &mut BcFunction, op: BcOp) -> bool {
    const_at(func, op).kind == BcVmConstKind::Number
  }

  fn as_number(&self, func: &mut BcFunction, op: BcOp) -> f64 {
    let vm_const = const_at(func, op);
    LUAU_ASSERT!(vm_const.kind == BcVmConstKind::Number);
    // Safety: 断言已限定活跃字段
    unsafe { vm_const.value.value_number }
  }

  fn as_imm(&self, func: &mut BcFunction, op: BcOp) -> BcImm {
    imm_at(func, op)
  }
}
