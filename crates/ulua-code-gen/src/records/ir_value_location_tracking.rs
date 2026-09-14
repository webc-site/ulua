use core::ffi::c_void;

use crate::records::{ir_function::IrFunction, ir_inst::IrInst};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrValueLocationTracking {
  pub function: *mut IrFunction,
  pub vm_reg_value: [u32; 256],
  pub vm_reg_dependent: [u32; 256],
  pub max_reg: i32,
  pub restore_callback_ctx: *mut c_void,
  // 恢复回调：设置方/调用方均在本 crate 内（Rust↔Rust），用 Rust ABI fn 指针
  pub restore_callback: Option<unsafe fn(*mut c_void, *mut IrInst)>,
}
