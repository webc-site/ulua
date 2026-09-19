use crate::records::{
  bc_function::BcFunction,
  sccp::{Sccp, SccpScratch, SccpState, VmConstOps},
};

/// cpp `foldConstants`：Sccp 入口——先播种 def→use 反向边（cpp 由 GraphParser
/// `addUse` 建图时维护），再传播常量格，最后按格值重写图。
pub fn sccp_fold_constants<I: VmConstOps + ?Sized>(func: &mut BcFunction, vm_ops: &I) {
  let mut sccp = Sccp {
    func,
    vm_ops,
    state: SccpState::new(),
    scratch: SccpScratch::default(),
  };
  sccp.seed_uses();
  sccp.propagate();
  sccp.rewrite();
}
