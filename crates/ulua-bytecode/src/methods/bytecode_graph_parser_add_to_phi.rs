use ulua_common::records::small_vector::SmallVector;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn add_to_phi(&mut self, block: BcOp, op: BcOp, proj: BcOp) -> BcOp {
    if op.kind == BcOpKind::Phi {
      let phi = self.func.phi_op(op);
      for &p in &phi.ops {
        if p == proj {
          return op;
        }
      }
      phi.ops.push_back(proj);
      op
    } else {
      let res = self.func.add_phi();
      // phi 归属于合并发生的目标块（对齐 cpp `makePhi(block, reg)`）
      self.func.block_op(block).phis.push(res);
      let phi = self.func.phi_op(res);
      phi.ops = SmallVector::from_iter([op, proj]);
      res
    }
  }
}
