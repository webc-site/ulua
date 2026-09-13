use core::ffi::c_void;
use std::collections::BTreeSet;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AreEqualState {
  pub(crate) seen: BTreeSet<(*const c_void, *const c_void)>,
  pub(crate) recursion_count: i32,
}

unsafe impl Send for AreEqualState {}
unsafe impl Sync for AreEqualState {}
