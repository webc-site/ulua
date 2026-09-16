//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/src/NonStrictTypeChecker.cpp:36:stack_pusher`
//! Source: `Analysis/src/NonStrictTypeChecker.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/NonStrictTypeChecker.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Def.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Common/include/Luau/TimeTrace.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/TypeUtils.h
//! - incoming:
//!   - declares <- source_file Analysis/src/NonStrictTypeChecker.cpp
//!   - type_ref <- method StackPusher::StackPusher (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- method StackPusher::operator= (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- method StackPusher::StackPusher (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- method NonStrictTypeChecker::pushStack (Analysis/src/NonStrictTypeChecker.cpp)
//!   - type_ref <- record StackPusher (Analysis/src/TypeChecker2.cpp)
//! - outgoing:
//!   - type_ref -> record StackPusher (Analysis/src/TypeChecker2.cpp)
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item StackPusher

extern crate alloc;

use alloc::vec::Vec;

use crate::records::scope::Scope;

// RAII scope-stack guard (NonStrictTypeChecker.cpp:36-65): pushes on
// construction, asserts-and-pops on drop. C++ is move-only with the moved-from
// guard neutered via std::exchange; Rust moves never run Drop on the source,
// so plain move semantics already match.
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
