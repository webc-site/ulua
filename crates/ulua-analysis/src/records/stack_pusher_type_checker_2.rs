extern crate alloc;

use alloc::vec::Vec;

use crate::records::scope::Scope;

// RAII scope-stack guard (TypeChecker2.cpp:61-86): pushes on construction,
// asserts-and-pops on drop. C++ is move-only with the moved-from guard neutered
// via std::exchange; Rust moves never run Drop on the source, so plain move
// semantics already match.
#[derive(Debug)]
pub struct StackPusher {
  pub stack: *mut Vec<*mut Scope>,
  pub scope: *mut Scope,
}

impl StackPusher {
  /// # Safety
  /// 调用方须保证 `stack、`scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn new(stack: *mut Vec<*mut Scope>, scope: *mut Scope) -> Self {
    unsafe {
      (*stack).push(scope);
    }
    Self { stack, scope }
  }
}

impl Drop for StackPusher {
  fn drop(&mut self) {
    if !self.stack.is_null() {
      unsafe {
        debug_assert_eq!((*self.stack).last().copied(), Some(self.scope));
        (*self.stack).pop();
      }
    }
  }
}
