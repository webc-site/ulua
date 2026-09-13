extern crate alloc;

use alloc::vec::Vec;

use ulua_common::records::small_vector::SmallVector;

use crate::records::{ir_op::IrOp, vm_exit_store_info::VmExitStoreInfo};

#[derive(Debug, Clone)]
pub struct VmExitSyncInfo {
  pub reg_stores: Vec<VmExitStoreInfo>,
  pub block: IrOp,
  pub vm_exit: IrOp,
  pub arg_ops: SmallVector<IrOp, 2>,
}
