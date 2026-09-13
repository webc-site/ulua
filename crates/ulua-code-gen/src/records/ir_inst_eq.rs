use ulua_common::records::dense_hash_table::DenseEq;

use crate::{enums::ir_op_kind::IrOpKind, records::ir_inst::IrInst};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrInstEq;

impl IrInstEq {
  #[inline]
  pub fn ir_inst_eq_operator_call(&self, a: &IrInst, b: &IrInst) -> bool {
    if a.cmd != b.cmd {
      return false;
    }

    let a_size = a.ops.size() as usize;
    let b_size = b.ops.size() as usize;

    if a_size == b_size {
      for i in 0..a_size {
        if !a.ops[i].ir_op_operator_eq(b.ops[i]) {
          return false;
        }
      }
    } else if a_size < b_size {
      let mut i: usize = 0;
      while i < a_size {
        if !a.ops[i].ir_op_operator_eq(b.ops[i]) {
          return false;
        }
        i += 1;
      }
      while i < b_size {
        if b.ops[i].kind() != IrOpKind::None {
          return false;
        }
        i += 1;
      }
    } else {
      let mut i: usize = 0;
      while i < b_size {
        if !a.ops[i].ir_op_operator_eq(b.ops[i]) {
          return false;
        }
        i += 1;
      }
      while i < a_size {
        if a.ops[i].kind() != IrOpKind::None {
          return false;
        }
        i += 1;
      }
    }

    true
  }
}

impl DenseEq<IrInst> for IrInstEq {
  fn eq(&self, a: &IrInst, b: &IrInst) -> bool {
    self.ir_inst_eq_operator_call(a, b)
  }
}
