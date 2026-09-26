use core::ptr::null_mut;

use ulua_analysis::records::scope::Scope;

use crate::{functions::raw_handle::raw_handle, records::normalize_fixture::NormalizeFixture};
impl NormalizeFixture {
  pub fn get_global_scope(&mut self) -> *mut Scope {
    self.get_frontend();
    self
      .global_scope
      .as_ref()
      .map(raw_handle)
      .unwrap_or(null_mut())
  }
}
