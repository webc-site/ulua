extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::ffi::c_void;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  cfg_info::CfgInfo, ir_block::IrBlock, ir_const::IrConst, vm_exit_sync_info::VmExitSyncInfo,
};

pub struct IrToStringContext<'a> {
  pub result: &'a mut String,
  pub blocks: &'a Vec<IrBlock>,
  pub constants: &'a Vec<IrConst>,
  pub cfg: &'a CfgInfo,
  pub vm_exit_info: &'a DenseHashMap<u32, VmExitSyncInfo>,
  pub proto: *mut c_void, // Proto is an opaque struct in this context
}
