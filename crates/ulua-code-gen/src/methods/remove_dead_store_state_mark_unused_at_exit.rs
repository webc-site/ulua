use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::remove_dead_store_state::RemoveDeadStoreState,
};

impl RemoveDeadStoreState {
  pub fn mark_unused_at_exit(&mut self, start: i32, count: i32) {
    CODEGEN_ASSERT!(count != 0);

    let e = if count == -1 {
      self.max_reg
    } else {
      start + count - 1
    };

    let function = unsafe { &*self.function };

    for i in start..=e {
      // Stores to captured registers are not removed since we don't track their uses outside of function
      if (function.cfg.captured.regs[i as usize / 64] & (1u64 << (i as usize % 64))) == 0 {
        self.info[i as usize].ignore_at_exit = true;
      }
    }
  }
}
