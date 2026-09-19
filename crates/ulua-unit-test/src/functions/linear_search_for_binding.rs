use alloc::string::String;

use ulua_analysis::{records::scope::Scope, type_aliases::type_id::TypeId};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn linear_search_for_binding(scope: *mut Scope, name: &str) -> Option<TypeId> {
  if scope.is_null() {
    return None;
  }

  unsafe {
    (*scope)
      .linear_search_for_binding(&String::from(name), true)
      .map(|binding| binding.type_id)
  }
}
