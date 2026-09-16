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

    let a_ops = a.ops.as_slice();
    let b_ops = b.ops.as_slice();

    // 公共前缀逐对相等
    if !a_ops
      .iter()
      .zip(b_ops)
      .all(|(x, &y)| x.ir_op_operator_eq(y))
    {
      return false;
    }

    // 较长一方的余量必须全部是 None 填充（None 与缺省操作数等价）
    let tail = if a_ops.len() >= b_ops.len() {
      &a_ops[b_ops.len()..]
    } else {
      &b_ops[a_ops.len()..]
    };
    tail.iter().all(|op| op.kind() == IrOpKind::None)
  }
}

impl DenseEq<IrInst> for IrInstEq {
  fn eq(&self, a: &IrInst, b: &IrInst) -> bool {
    self.ir_inst_eq_operator_call(a, b)
  }
}
