use crate::records::in_conditional_context::InConditionalContext;

impl Drop for InConditionalContext {
  fn drop(&mut self) {
    unsafe {
      *self.type_context = self.old_value;
    }
  }
}
