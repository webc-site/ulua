use ulua_analysis::{records::scope::Scope, type_aliases::type_id::TypeId};

use crate::{functions::lookup_name::lookup_name, records::fixture::Fixture};

impl Fixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn require_type_scope_ptr_string(&mut self, scope: *mut Scope, name: &str) -> TypeId {
    let ty = unsafe { lookup_name(scope, name) };
    if ty.is_none() {
      panic!("requireType: No type \"{}\"", name);
    }
    ty.unwrap()
  }
}
