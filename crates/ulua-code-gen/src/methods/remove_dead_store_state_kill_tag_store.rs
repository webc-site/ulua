use crate::{
  functions::kill_ir_utils::kill_ir_function_ir_inst,
  records::{
    ir_inst::IrInst, remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo,
  },
};

impl RemoveDeadStoreState {
  pub fn kill_tag_store(&mut self, reg_info: &mut StoreRegInfo) {
    if reg_info.tag_inst_idx != !0u32 {
      let function = unsafe { &mut *self.function };
      let inst_ptr: *mut IrInst = &mut function.instructions[reg_info.tag_inst_idx as usize];
      kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });

      reg_info.tag_inst_idx = !0u32;
      reg_info.maybe_gco = false;
    }
  }
}
