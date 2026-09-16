use crate::records::{global_state::global_State, scoped_set_gc_threshold::ScopedSetGcThreshold};

impl ScopedSetGcThreshold {
  /// # Safety
  ///
  /// `global` must point to a valid `global_State`.
  pub(crate) unsafe fn scoped_set_gc_threshold_global_state_usize(
    &mut self,
    global: *mut global_State,
    new_threshold: usize,
  ) {
    self.global = global;
    unsafe {
      self.original_threshold = (*global).gc_threshold;
      (*global).gc_threshold = new_threshold;
    }
  }
}
