use crate::records::bc_op::BcOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LoopInfo {
  pub(crate) entry: BcOp,
  pub(crate) exit: BcOp,
}
