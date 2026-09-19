use ulua_common::records::small_vector::SmallVector;

use crate::records::bc_op::BcOp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BcPhi {
  pub ops: SmallVector<BcOp, 4>,
}
