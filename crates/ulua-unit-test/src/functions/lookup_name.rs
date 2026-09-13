use ulua_analysis::{records::scope::Scope, type_aliases::type_id::TypeId};

use crate::functions::linear_search_for_binding::linear_search_for_binding;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn lookup_name(scope: *mut Scope, name: &str) -> Option<TypeId> {
  unsafe { linear_search_for_binding(scope, name) }
}
