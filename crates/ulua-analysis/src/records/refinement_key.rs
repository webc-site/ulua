use alloc::string::String;
/// `DefId` is a `NotNull<const Def>`, which in Rust is represented as a non-null raw pointer to a `Def`.
use core::ffi::c_void;
use core::ptr::null;
pub type DefId = *const c_void;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefinementKey {
  pub(crate) parent: *const RefinementKey,
  pub(crate) def: DefId,
  pub(crate) prop_name: Option<String>,
}

impl Default for RefinementKey {
  fn default() -> Self {
    Self {
      parent: null(),
      def: null(),
      prop_name: None,
    }
  }
}

impl RefinementKey {
  pub fn parent(&self) -> *const RefinementKey {
    self.parent
  }

  pub fn def(&self) -> DefId {
    self.def
  }

  pub fn prop_name(&self) -> Option<&String> {
    self.prop_name.as_ref()
  }
}
