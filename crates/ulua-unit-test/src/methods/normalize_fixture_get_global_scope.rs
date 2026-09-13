//! @interface-stub
use alloc::sync::Arc;
use core::ptr::null_mut;

use ulua_analysis::records::scope::Scope;

use crate::records::normalize_fixture::NormalizeFixture;
impl NormalizeFixture {
  pub fn get_global_scope(&mut self) -> *mut Scope {
    self.get_frontend();
    self
      .global_scope
      .as_ref()
      .map(|scope| Arc::as_ptr(scope) as *mut Scope)
      .unwrap_or(null_mut())
  }
}
