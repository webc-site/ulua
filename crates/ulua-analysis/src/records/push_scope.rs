use crate::type_aliases::scope_stack::ScopeStack;

#[derive(Debug)]
pub struct PushScope {
  pub(crate) stack: *mut ScopeStack,
  pub(crate) previous_size: usize,
}

impl Drop for PushScope {
  fn drop(&mut self) {
    self.pop();
  }
}
