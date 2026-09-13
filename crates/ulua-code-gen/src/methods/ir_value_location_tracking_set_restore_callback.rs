use core::ffi::c_void;

use crate::records::{ir_inst::IrInst, ir_value_location_tracking::IrValueLocationTracking};

impl IrValueLocationTracking {
  pub fn set_restore_callback(
    &mut self,
    context: *mut c_void,
    callback: Option<unsafe fn(*mut c_void, *mut IrInst)>,
  ) {
    self.restore_callback_ctx = context;
    self.restore_callback = callback;
  }
}
