use crate::records::{
  bc_function::BcFunction,
  sccp::{Sccp, SccpState, VmConstOps},
};

/// cpp `foldConstants`：Sccp 入口——先传播常量格，再按格值重写图。
pub fn sccp_fold_constants<I: VmConstOps + ?Sized>(func: &mut BcFunction, vm_ops: &I) {
  let mut sccp = Sccp {
    func,
    vm_ops,
    state: SccpState::new(),
  };
  sccp.propagate();
  sccp.rewrite();
}
