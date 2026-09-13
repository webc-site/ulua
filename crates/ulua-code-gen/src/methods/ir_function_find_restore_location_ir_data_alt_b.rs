use crate::records::{
  ir_function::IrFunction, ir_inst::IrInst, value_restore_location::ValueRestoreLocation,
};

impl IrFunction {
  pub fn find_restore_location_ir_inst_bool(
    &self,
    inst: &IrInst,
    limit_to_current_block: bool,
  ) -> ValueRestoreLocation {
    self.find_restore_location_u32_bool(self.get_inst_index(inst), limit_to_current_block)
  }
}
