use core::ptr::null_mut;

use crate::records::{
  ir_data::K_INVALID_INST_IDX, ir_function::IrFunction,
  ir_value_location_tracking::IrValueLocationTracking,
};

impl IrValueLocationTracking {
  pub fn new(function: &mut IrFunction) -> Self {
    let mut tracking = Self {
      function: function as *mut IrFunction,
      vm_reg_value: [K_INVALID_INST_IDX; 256],
      vm_reg_dependent: [K_INVALID_INST_IDX; 256],
      max_reg: 0,
      restore_callback_ctx: null_mut(),
      restore_callback: None,
    };

    // Mirror the C++ constructor explicitly initializing the arrays.
    tracking.vm_reg_value.fill(K_INVALID_INST_IDX);
    tracking.vm_reg_dependent.fill(K_INVALID_INST_IDX);

    tracking
  }
}
