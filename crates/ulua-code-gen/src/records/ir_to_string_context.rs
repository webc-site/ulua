extern crate alloc;

use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::records::proto::Proto;

use crate::records::{
  cfg_info::CfgInfo, ir_block::IrBlock, ir_const::IrConst, vm_exit_sync_info::VmExitSyncInfo,
};

pub struct IrToStringContext<'a> {
  pub result: &'a mut String,
  pub blocks: &'a Vec<IrBlock>,
  pub constants: &'a Vec<IrConst>,
  pub cfg: &'a CfgInfo,
  pub vm_exit_info: &'a DenseHashMap<u32, VmExitSyncInfo>,
  /// 宿主 `Proto`（dump 时展开 VM 常量用）。`IrFunction::proto` 允许为空（无字节码来源的
  /// IR），此处用 `Option<&Proto>` 取代原先的 `*mut c_void` 类型擦除裸指针（review.md §2）。
  pub proto: Option<&'a Proto>,
}
