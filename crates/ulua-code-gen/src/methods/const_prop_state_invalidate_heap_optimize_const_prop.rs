use ulua_common::FFlag::LuauCodegenExtraTableOpts;

use crate::records::{const_prop_state::ConstPropState, register_info::RegisterInfo};
impl ConstPropState {
  pub fn invalidate_heap(&mut self) {
    if LuauCodegenExtraTableOpts.get() {
      self.inst_not_readonly.clear();
      self.inst_no_metatable.clear();
      self.inst_array_size.clear();
    } else {
      let max_reg = self.max_reg;
      for i in 0..=max_reg {
        let idx = i as usize;
        let reg_ptr: *mut RegisterInfo = &mut self.regs[idx];
        unsafe {
          self.invalidate_heap_register_info(&mut *reg_ptr);
        }
      }
    }

    self.invalidate_heap_table_data();

    self.buffer_load_store_info.clear();
  }
}
