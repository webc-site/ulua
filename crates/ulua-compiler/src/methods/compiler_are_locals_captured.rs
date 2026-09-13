use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn are_locals_captured(&mut self, start: usize) -> bool {
    LUAU_ASSERT!(start <= self.local_stack.len());

    for i in start..self.local_stack.len() {
      let local = self.local_stack[i];
      let l = self.locals.find(&local);
      LUAU_ASSERT!(l.is_some());

      if l.is_some_and(|l| l.captured) {
        return true;
      }
    }

    false
  }
}
