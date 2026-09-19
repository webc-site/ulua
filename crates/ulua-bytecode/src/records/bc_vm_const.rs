use core::fmt::{self, Debug, Formatter};

use crate::enums::bc_vm_const_kind::BcVmConstKind;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BcVmConst {
  pub kind: BcVmConstKind,
  pub value: BcVmConstValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BcVmConstValue {
  pub value_boolean: bool,
  pub value_number: f64,
  /// cpp `valueVectorf`
  pub value_vector: [f32; 4],
  /// cpp `valueVectord`（`BytecodeGraph.h:160`）
  pub value_vectord: [f64; 4],
  pub value_string: &'static str,
  pub value_import: u32,
  pub value_table: u32,
  pub value_closure: u32,
  pub value_integer: i64,
  /// cpp: `valueClassShape`（`cpp/Bytecode/include/Luau/BytecodeGraph.h:166`）
  pub value_class_shape: u32,
}

impl Debug for BcVmConstValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("BcVmConstValue(..)")
  }
}

/// 语义对齐 cpp `BcVmConst::operator==`（BytecodeGraph.h）：先比 kind，再按 kind 读 union 对应字段。
impl PartialEq for BcVmConst {
  fn eq(&self, other: &Self) -> bool {
    if self.kind != other.kind {
      return false;
    }
    // Safety: 两侧 kind 相等，按 kind 读取 union 的对应字段属于活跃字段
    unsafe {
      match self.kind {
        BcVmConstKind::Nil => true,
        BcVmConstKind::Boolean => self.value.value_boolean == other.value.value_boolean,
        BcVmConstKind::Number => self.value.value_number == other.value.value_number,
        BcVmConstKind::Vector => self.value.value_vector == other.value.value_vector,
        // cpp: `BytecodeGraph.h:195-198`
        BcVmConstKind::Vectord => self.value.value_vectord == other.value.value_vectord,
        BcVmConstKind::String => self.value.value_string == other.value.value_string,
        BcVmConstKind::Import => self.value.value_import == other.value.value_import,
        BcVmConstKind::Table => self.value.value_table == other.value.value_table,
        BcVmConstKind::Closure => self.value.value_closure == other.value.value_closure,
        BcVmConstKind::Integer => self.value.value_integer == other.value.value_integer,
        // cpp: `cpp/Bytecode/include/Luau/BytecodeGraph.h:214`
        BcVmConstKind::ClassShape => self.value.value_class_shape == other.value.value_class_shape,
      }
    }
  }
}

impl Eq for BcVmConst {}
