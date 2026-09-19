use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::push_scope::PushScope;

impl PushScope {
  pub fn pop(&mut self) {
    if self.previous_size == usize::MAX {
      return;
    }

    // If somehow this stack has _shrunk_ to be smaller than we expect,
    // something very strange has happened.
    let stack_ref = unsafe { &*self.stack };
    LUAU_ASSERT!(stack_ref.len() > self.previous_size);

    let stack_mut = unsafe { &mut *self.stack };
    while stack_mut.len() > self.previous_size {
      stack_mut.pop();
    }
    self.previous_size = usize::MAX;
  }
}
