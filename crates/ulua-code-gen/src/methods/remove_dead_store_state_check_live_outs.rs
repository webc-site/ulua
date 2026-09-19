use crate::records::{
  ir_block::IrBlock, ir_function::IrFunction, remove_dead_store_state::RemoveDeadStoreState,
  store_reg_info::StoreRegInfo,
};

impl RemoveDeadStoreState {
  // When checking block terminators, any registers that are not live out can be removed by saying that a new value is being 'defined'
  pub fn check_live_outs(&mut self, block: &IrBlock) {
    let function: *mut IrFunction = self.function;

    let index = unsafe { (*function).get_block_index(block) };

    if (index as usize) < unsafe { (*function).cfg.out.len() } {
      let max_reg = self.max_reg;

      for i in 0..=max_reg {
        let out = unsafe { &(&(*function).cfg.out)[index as usize] };

        let is_out = (out.regs[i as usize / 64] & (1u64 << (i as usize % 64))) != 0
          || (out.vararg_seq && i >= out.vararg_start as i32);

        if !is_out {
          // Stores to captured registers are not removed since we don't track their uses outside of function
          let captured = (unsafe { (*function).cfg.captured.regs[i as usize / 64] }
            & (1u64 << (i as usize % 64)))
            != 0;

          if !captured {
            let reg_info: &mut StoreRegInfo =
              unsafe { &mut *(&mut self.info[i as usize] as *mut StoreRegInfo) };
            self.kill_tag_and_value_store_pair(reg_info);
            self.kill_t_value_store(reg_info);
          }
        }
      }
    }
  }
}
