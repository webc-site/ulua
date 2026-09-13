use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::scoped_spills::ScopedSpills};

impl Drop for ScopedSpills {
  fn drop(&mut self) {
    if self.owner.is_null() {
      return;
    }

    let owner = unsafe { &mut *self.owner };
    let end_spill_id = owner.next_spill_id;

    let mut i = 0;
    while i < owner.spills.len() {
      let spill = &owner.spills[i];

      // Restoring spills inside this scope cannot create new spills.
      CODEGEN_ASSERT!(spill.spill_id < end_spill_id);

      if spill.spill_id >= self.start_spill_id {
        let inst_idx = spill.inst_idx as usize;
        let inst = unsafe { &mut *(*owner.function).instructions.as_mut_ptr().add(inst_idx) };

        owner.restore(inst, true);
      } else {
        i += 1;
      }
    }
  }
}
