//! IR 操作数访问器族：C++ `OP_x`/`OPT_OP_x` 宏的 Rust 对偶
//! （按值/只读形态，越界缺省语义逐字保真）。
//! （r7-macros98 合并票：仅搬家不改货。）

use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::get_op_ir_data::get_op_mut,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

/// C++ 的 `OP_A(inst)`——指令的第一个操作数。
#[inline]
pub fn op_a(inst: &mut IrInst) -> IrOp {
  *get_op_mut(inst, 0)
}

/// C++ `OP_A` 只读形态：越界时返回缺省操作数
/// （`op_a` 引用版越界时 resize 原指令，只读调用点不应有此副作用）
#[inline]
pub fn op_a_ref(inst: &IrInst) -> IrOp {
  if 0 < inst.ops.size() {
    inst.ops[0]
  } else {
    IrOp::default()
  }
}

#[inline]
pub fn op_b(mut inst: IrInst) -> IrOp {
  *get_op_mut(&mut inst, 1)
}

/// C++ `OP_B` 只读形态：越界时返回缺省操作数
/// （按值版 `op_b` 克隆体上的 resize 外部不可见，故语义等价）
#[inline]
pub fn op_b_ref(inst: &IrInst) -> IrOp {
  if 1 < inst.ops.size() {
    inst.ops[1]
  } else {
    IrOp::default()
  }
}

/// C++ `OP_C` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_C` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_c_ref(inst: &IrInst) -> IrOp {
  if 2 < inst.ops.size() {
    inst.ops[2]
  } else {
    IrOp::default()
  }
}

/// C++ `OP_D` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_D` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_d_ref(inst: &IrInst) -> IrOp {
  if 3 < inst.ops.size() {
    inst.ops[3]
  } else {
    IrOp::default()
  }
}

/// C++ `OP_E` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_E` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_e_ref(inst: &IrInst) -> IrOp {
  if 4 < inst.ops.size() {
    inst.ops[4]
  } else {
    IrOp::default()
  }
}

/// C++ `OP_F` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_F` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_f_ref(inst: &IrInst) -> IrOp {
  if 5 < inst.ops.size() {
    inst.ops[5]
  } else {
    IrOp::default()
  }
}

/// C++ `OP_G` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_G` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_g_ref(inst: &IrInst) -> IrOp {
  if 6 < inst.ops.size() {
    inst.ops[6]
  } else {
    IrOp::default()
  }
}

/// C++ `OPT_OP_B` 只读形态：缺槽或 None 时返回空操作数
#[inline]
pub fn opt_op_b_ref(inst: &IrInst) -> IrOp {
  if 1 < inst.ops.size() && inst.ops[1].kind() != IrOpKind::None {
    inst.ops[1]
  } else {
    IrOp { kind_and_index: 0 }
  }
}

/// C++ `OPT_OP_C` 只读形态：缺槽或 None 时返回空操作数
#[inline]
pub fn opt_op_c_ref(inst: &IrInst) -> IrOp {
  if 2 < inst.ops.size() && inst.ops[2].kind() != IrOpKind::None {
    inst.ops[2]
  } else {
    IrOp { kind_and_index: 0 }
  }
}

/// C++ `OPT_OP_D` 只读形态：缺槽或 None 时返回空操作数
#[inline]
pub fn opt_op_d_ref(inst: &IrInst) -> IrOp {
  if 3 < inst.ops.size() && inst.ops[3].kind() != IrOpKind::None {
    inst.ops[3]
  } else {
    IrOp { kind_and_index: 0 }
  }
}

// HAS_OP_x 操作数存在性宏族（r7-useline 票面 A：自 has_op_b/c/d/e.rs 碎片迁回，
// 宏体逐字保真，黑名单消费方仅改 use 路径）。

/// Source: `CodeGen/include/Luau/IrData.h:1218` (hand-ported)
// #define HAS_OP_B(inst) (1 < (inst).ops.size() && (inst).ops[1].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_B {
  ($inst:expr) => {
    1 < ($inst).ops.size() as usize
      && ($inst).ops[1].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}

/// Source: `CodeGen/include/Luau/IrData.h:1219` (hand-ported)
// #define HAS_OP_C(inst) (2 < (inst).ops.size() && (inst).ops[2].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_C {
  ($inst:expr) => {
    2 < ($inst).ops.size() as usize
      && ($inst).ops[2].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}

/// Source: `CodeGen/include/Luau/IrData.h:1220` (hand-ported)
// #define HAS_OP_D(inst) (3 < (inst).ops.size() && (inst).ops[3].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_D {
  ($inst:expr) => {
    3 < ($inst).ops.size() as usize
      && ($inst).ops[3].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}

/// Source: `CodeGen/include/Luau/IrData.h:1221` (hand-ported)
// #define HAS_OP_E(inst) (4 < (inst).ops.size() && (inst).ops[4].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_E {
  ($inst:expr) => {
    4 < ($inst).ops.size() as usize
      && ($inst).ops[4].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}

pub use {HAS_OP_B, HAS_OP_C, HAS_OP_D, HAS_OP_E};
